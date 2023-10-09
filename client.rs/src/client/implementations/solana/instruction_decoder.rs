use {
    anchor_client::anchor_lang::{AnchorDeserialize, AnchorSerialize},
    anyhow::{bail, Result},
    enum_extract::let_extract,
    hapi_core_solana::Category,
    sha2::{Digest, Sha256},
    solana_transaction_status::{
        EncodedConfirmedTransactionWithStatusMeta, EncodedTransaction, UiCompiledInstruction,
        UiMessage,
    },
    std::collections::HashMap,
};

use crate::HapiCoreSolana;

/// Struct representing an Instruction entity from a Solana transaction
pub struct Instruction {
    /// Sequence index in transaction
    pub id: u8,

    /// Transaction signature hash
    pub tx_hash: String,

    /// The public key of the account containing a program
    pub program_id: String,

    /// Time of transaction block
    pub blocktime: u64,

    /// List of encoded accounts used by the instruction
    pub account_keys: Vec<String>,

    /// The program input data encoded in a base-58 string
    pub data: InstructionData,
}

/// Byte index of bump in account data
const DISCRIMINATOR_SIZE: usize = 8;

#[derive(AnchorSerialize, AnchorDeserialize, PartialEq)]
pub struct CreateAddressData {
    pub address: [u8; 64],
    pub category: Category,
    pub risk: u8,
    pub bump: u8,
}

#[derive(AnchorSerialize, AnchorDeserialize, PartialEq)]
pub struct UpdateAddressData {
    pub category: Category,
    pub risk: u8,
}

pub enum InstructionData {
    CreateAddress(CreateAddressData),
    UpdateAddress(UpdateAddressData),
}

impl HapiCoreSolana {
    fn get_instructions(
        &self,
        tx: EncodedConfirmedTransactionWithStatusMeta,
    ) -> Result<Vec<Instruction>> {
        let_extract!(
            EncodedTransaction::Json(json_tx),
            tx.transaction.transaction,
            bail!("Wrong transaction encoding")
        );
        let_extract!(
            UiMessage::Raw(msg),
            json_tx.message,
            bail!("Wrong message encoding")
        );

        if msg.account_keys.is_empty() {
            bail!("Empty transaction accounts")
        }

        let_extract!(
            Some(tx_hash),
            json_tx.signatures.first(),
            bail!("Tx without signature")
        );
        let_extract!(
            Some(blocktime),
            tx.block_time,
            bail!("Tx without blocktime")
        );

        let mut result = vec![];
        for (id, instr) in msg.instructions.iter().enumerate() {
            if let Some(instruction) = self.parse_instruction(
                id as u8,
                instr,
                &msg.account_keys,
                tx_hash.clone(),
                blocktime as u64,
            )? {
                result.push(instruction);
            }
        }

        Ok(result)
    }

    fn parse_instruction(
        &self,
        id: u8,
        instruction: &UiCompiledInstruction,
        tx_accounts: &Vec<String>,
        tx_hash: String,
        blocktime: u64,
    ) -> Result<Option<Instruction>> {
        if instruction.accounts.is_empty() {
            bail!("Empty instruction accounts")
        }

        let instruction_program_id = &tx_accounts[instruction.program_id_index as usize];

        if instruction_program_id == &self.program_id.to_string() {
            let buf = &bs58::decode(&instruction.data).into_vec()?;

            let data = {
                let data_slice = &buf[DISCRIMINATOR_SIZE..];

                match &buf[..DISCRIMINATOR_SIZE] {
                    create_addr_sighash => InstructionData::CreateAddress(
                        CreateAddressData::try_from_slice(data_slice)?,
                    ),
                    update_addr_sighash => InstructionData::UpdateAddress(
                        UpdateAddressData::try_from_slice(data_slice)?,
                    ),
                    _ => return Ok(None),
                }
            };

            let account_keys = instruction
                .accounts
                .iter()
                .map(|&index| tx_accounts[index as usize].clone())
                .collect::<Vec<_>>();

            return Ok(Some(Instruction {
                id: id as u8,
                tx_hash,
                program_id: instruction_program_id.to_string(),
                blocktime,
                account_keys,
                data,
            }));
        }

        Ok(None)
    }
}

/// Hashes instruction name to bytearray
pub(crate) fn get_instruction_sighashes() -> HashMap<&'static str, [u8; 8]> {
    let names = ["create_address", "update_address"];

    HashMap::from_iter(names.iter().map(|&name| {
        let mut hasher = Sha256::new();
        hasher.update(format!("global:{}", name).as_bytes());

        let mut sighash = [0u8; DISCRIMINATOR_SIZE];
        sighash.copy_from_slice(&hasher.finalize()[..DISCRIMINATOR_SIZE]);
        (name, sighash)
    }))
}
