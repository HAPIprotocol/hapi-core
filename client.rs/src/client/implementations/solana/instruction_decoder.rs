use {
    anchor_client::{anchor_lang::AnchorDeserialize, solana_sdk::signature::Signature},
    anyhow::{bail, Result},
    enum_extract::let_extract,
    hapi_core_solana::{RewardConfiguration, StakeConfiguration},
    solana_transaction_status::{
        EncodedConfirmedTransactionWithStatusMeta, EncodedTransaction, UiCompiledInstruction,
        UiMessage, UiTransactionEncoding,
    },
    std::str::FromStr,
};

use super::instructions::{
    CreateAddressData, CreateAssetData, CreateCaseData, CreateNetworkData, CreateReporterData,
    HapiInstruction, InstructionData, UpdateAddressData, UpdateAssetData, UpdateCaseData,
    UpdateReporterData, DISCRIMINATOR_SIZE,
};
use crate::{client::result::ClientError, HapiCoreSolana};

/// Struct representing an Instruction entity from a Solana transaction
pub struct DecodedInstruction {
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

impl HapiCoreSolana {
    pub async fn get_instructions(&self, hash: &str) -> Result<Vec<DecodedInstruction>> {
        let tx = self
            .rpc_client
            .get_transaction(&Signature::from_str(hash)?, UiTransactionEncoding::Json)
            .await?;

        Ok(self
            .decode_transaction(tx)
            .map_err(|e| ClientError::InstructionDecodingError(e.to_string()))?)
    }

    fn decode_transaction(
        &self,
        tx: EncodedConfirmedTransactionWithStatusMeta,
    ) -> Result<Vec<DecodedInstruction>> {
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
    ) -> Result<Option<DecodedInstruction>> {
        if instruction.accounts.is_empty() {
            bail!("Empty instruction accounts")
        }

        let instruction_program_id = &tx_accounts[instruction.program_id_index as usize];

        if instruction_program_id == &self.program_id.to_string() {
            let buf = &bs58::decode(&instruction.data).into_vec()?;

            let data = {
                let sighash = &buf[..DISCRIMINATOR_SIZE];
                let data_slice = &buf[DISCRIMINATOR_SIZE..];

                if sighash == self.hashes[HapiInstruction::CreateNetwork as usize] {
                    InstructionData::CreateNetwork(CreateNetworkData::try_from_slice(data_slice)?)
                } else if sighash == self.hashes[HapiInstruction::UpdateStakeConfiguration as usize]
                {
                    InstructionData::UpdateStakeConfiguration(StakeConfiguration::try_from_slice(
                        data_slice,
                    )?)
                } else if sighash
                    == self.hashes[HapiInstruction::UpdateRewardConfiguration as usize]
                {
                    InstructionData::UpdateRewardConfiguration(RewardConfiguration::try_from_slice(
                        data_slice,
                    )?)
                } else if sighash == self.hashes[HapiInstruction::SetAuthority as usize] {
                    InstructionData::SetAuthority()
                } else if sighash == self.hashes[HapiInstruction::CreateReporter as usize] {
                    InstructionData::CreateReporter(CreateReporterData::try_from_slice(data_slice)?)
                } else if sighash == self.hashes[HapiInstruction::UpdateReporter as usize] {
                    InstructionData::UpdateReporter(UpdateReporterData::try_from_slice(data_slice)?)
                } else if sighash == self.hashes[HapiInstruction::ActivateReporter as usize] {
                    InstructionData::ActivateReporter()
                } else if sighash == self.hashes[HapiInstruction::DeactivateReporter as usize] {
                    InstructionData::DeactivateReporter()
                } else if sighash == self.hashes[HapiInstruction::Unstake as usize] {
                    InstructionData::Unstake()
                } else if sighash == self.hashes[HapiInstruction::CreateCase as usize] {
                    InstructionData::CreateCase(CreateCaseData::try_from_slice(data_slice)?)
                } else if sighash == self.hashes[HapiInstruction::UpdateCase as usize] {
                    InstructionData::UpdateCase(UpdateCaseData::try_from_slice(data_slice)?)
                } else if sighash == self.hashes[HapiInstruction::CreateAddress as usize] {
                    InstructionData::CreateAddress(CreateAddressData::try_from_slice(data_slice)?)
                } else if sighash == self.hashes[HapiInstruction::UpdateAddress as usize] {
                    InstructionData::UpdateAddress(UpdateAddressData::try_from_slice(data_slice)?)
                } else if sighash == self.hashes[HapiInstruction::ConfirmAddress as usize] {
                    InstructionData::ConfirmAddress(u8::try_from_slice(data_slice)?)
                } else if sighash == self.hashes[HapiInstruction::CreateAsset as usize] {
                    InstructionData::CreateAsset(CreateAssetData::try_from_slice(data_slice)?)
                } else if sighash == self.hashes[HapiInstruction::UpdateAsset as usize] {
                    InstructionData::UpdateAsset(UpdateAssetData::try_from_slice(data_slice)?)
                } else if sighash == self.hashes[HapiInstruction::ConfirmAsset as usize] {
                    InstructionData::ConfirmAsset(u8::try_from_slice(data_slice)?)
                } else {
                    return Ok(None);
                }
            };

            let account_keys = instruction
                .accounts
                .iter()
                .map(|&index| tx_accounts[index as usize].clone())
                .collect::<Vec<_>>();

            return Ok(Some(DecodedInstruction {
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

#[cfg(test)]
mod tests {
    use {
        anchor_client::anchor_lang::AnchorSerialize,
        hapi_core_solana::Category,
        solana_transaction_status::{
            EncodedConfirmedTransactionWithStatusMeta, EncodedTransaction,
            EncodedTransactionWithStatusMeta, UiMessage, UiRawMessage, UiTransaction,
        },
        spl_token::solana_program::message::MessageHeader,
    };

    use crate::{
        client::solana::instructions::get_instruction_sighash, HapiCoreNetwork, HapiCoreOptions,
    };

    use super::*;

    const NETWORK: &str = "QDWdYo5JWQ96cCEgdBXpL6TVs5whScFSzVbZgobHLrQ";
    const COMMUNITY: &str = "C7DNJUKfDVpL9ZZqLnVTG1adj4Yu46JgDB6hiTdMEktX";
    const ADDRESS: &str = "WN4cDdcxEEzCVyaFEuG4zzJB6QNqrahtfYpSeeecrmC";
    const PROGRAM_ID: &str = "39WzZqJgkK2QuQxV9jeguKRgHE65Q3HywqPwBzdrKn2B";

    fn serialize<T: AnchorSerialize>(name: &str, data: T) -> Vec<u8> {
        let mut instruction_data = get_instruction_sighash(name).to_vec();

        let mut inner_data = Vec::new();
        data.serialize(&mut instruction_data)
            .expect("Failed to serialize update address data");

        instruction_data.append(&mut inner_data);
        instruction_data
    }

    fn create_instruction(name: &str, data: &InstructionData) -> UiCompiledInstruction {
        let instruction_data = match data {
            InstructionData::CreateNetwork(data) => serialize(name, data),
            InstructionData::UpdateStakeConfiguration(data) => serialize(name, data),
            InstructionData::UpdateRewardConfiguration(data) => serialize(name, data),
            InstructionData::CreateReporter(data) => serialize(name, data),
            InstructionData::UpdateReporter(data) => serialize(name, data),
            InstructionData::CreateCase(data) => serialize(name, data),
            InstructionData::UpdateCase(data) => serialize(name, data),
            InstructionData::CreateAddress(data) => serialize(name, data),
            InstructionData::UpdateAddress(data) => serialize(name, data),
            InstructionData::ConfirmAddress(data) => serialize(name, data),
            InstructionData::CreateAsset(data) => serialize(name, data),
            InstructionData::UpdateAsset(data) => serialize(name, data),
            InstructionData::ConfirmAsset(data) => serialize(name, data),
            _ => get_instruction_sighash(name).to_vec(),
        };

        UiCompiledInstruction {
            program_id_index: 0,
            accounts: vec![0, 1, 2, 3],
            data: bs58::encode(instruction_data).into_string(),
            stack_height: None,
        }
    }

    fn create_tx(data: &Vec<(&str, InstructionData)>) -> EncodedConfirmedTransactionWithStatusMeta {
        let account_keys = vec![
            String::from(PROGRAM_ID),
            String::from(COMMUNITY),
            String::from(NETWORK),
            String::from(ADDRESS),
        ];

        let instructions = data
            .iter()
            .map(|(name, data)| create_instruction(name, data))
            .collect();

        EncodedConfirmedTransactionWithStatusMeta {

            slot: 123,
            transaction: EncodedTransactionWithStatusMeta {
                transaction: EncodedTransaction::Json(UiTransaction {
                    signatures: vec!["3AsdoALgZFuq2oUVWrDYhg2pNeaLJKPLf8hU2mQ6U8qJxeJ6hsrPVpMn9ma39DtfYCrDQSvngWRP8NnTpEhezJpE".to_string()],
                    message: UiMessage::Raw(UiRawMessage {
                        header: MessageHeader {
                            num_required_signatures: 1,
                            num_readonly_signed_accounts: 1,
                            num_readonly_unsigned_accounts: 2,
                        },
                        account_keys,
                        recent_blockhash: String::default(),
                        instructions,
                        address_table_lookups: None,
                    }),
                }),
                meta: None,
                version: None,
            },
            block_time: Some(123),
        }
    }

    fn get_cli(program_id: Option<String>) -> HapiCoreSolana {
        HapiCoreSolana::new(HapiCoreOptions {
            provider_url: String::default(),
            contract_address: program_id.unwrap_or(PROGRAM_ID.to_string()),
            private_key: None,
            chain_id: None,
            network: HapiCoreNetwork::Solana,
        })
        .expect("Failed to initialize client")
    }

    #[test]
    fn decode_hapi_transactions() {
        let client = get_cli(None);

        let test_data = vec![
            (
                "create_network",
                InstructionData::CreateNetwork(CreateNetworkData::default()),
            ),
            (
                "update_stake_configuration",
                InstructionData::UpdateStakeConfiguration(StakeConfiguration::default()),
            ),
            (
                "update_reward_configuration",
                InstructionData::UpdateRewardConfiguration(RewardConfiguration::default()),
            ),
            ("set_authority", InstructionData::SetAuthority()),
            (
                "create_reporter",
                InstructionData::CreateReporter(CreateReporterData::default()),
            ),
            (
                "update_reporter",
                InstructionData::UpdateReporter(UpdateReporterData::default()),
            ),
            ("activate_reporter", InstructionData::ActivateReporter()),
            ("deactivate_reporter", InstructionData::DeactivateReporter()),
            ("unstake", InstructionData::Unstake()),
            (
                "create_case",
                InstructionData::CreateCase(CreateCaseData::default()),
            ),
            (
                "update_case",
                InstructionData::UpdateCase(UpdateCaseData::default()),
            ),
            (
                "create_address",
                InstructionData::CreateAddress(CreateAddressData {
                    address: [1u8; 64],
                    category: Category::Gambling,
                    risk: 5,
                    bump: 255,
                }),
            ),
            (
                "update_address",
                InstructionData::UpdateAddress(UpdateAddressData::default()),
            ),
            ("confirm_address", InstructionData::ConfirmAddress(255)),
            (
                "create_asset",
                InstructionData::CreateAsset(CreateAssetData {
                    addr: [1u8; 64],
                    asset_id: [1u8; 64],
                    category: Category::ATM,
                    risk_score: 5,
                    bump: 255,
                }),
            ),
            (
                "update_asset",
                InstructionData::UpdateAsset(UpdateAssetData::default()),
            ),
            ("confirm_asset", InstructionData::ConfirmAsset(255)),
        ];

        let instructions = client
            .decode_transaction(create_tx(&test_data))
            .expect("Failed to decode transaction");

        assert_eq!(instructions.len(), test_data.len());

        for (index, instruction) in instructions.iter().enumerate() {
            assert_eq!(instruction.data, test_data[index].1);
            assert_eq!(&instruction.program_id, PROGRAM_ID);
        }
    }

    #[test]
    fn ignore_unknown_instruction() {
        let client = get_cli(None);

        let instructions = client
            .decode_transaction(create_tx(&vec![(
                "unknown_instruction",
                InstructionData::Unstake(),
            )]))
            .expect("Failed to decode transaction");

        assert_eq!(instructions.len(), 0);
    }

    #[test]
    fn ignore_invalid_program_id() {
        let client = get_cli(Some(
            "9ZNTfG4NyQgxy2SWjSiQoUyBPEvXT2xo7fKc5hPYYJ7b".to_string(),
        ));

        let instructions = client
            .decode_transaction(create_tx(&vec![("unstake", InstructionData::Unstake())]))
            .expect("Failed to decode transaction");

        assert_eq!(instructions.len(), 0);
    }
}
