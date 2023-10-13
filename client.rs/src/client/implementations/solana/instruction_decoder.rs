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

use super::instruction_data::{
    CreateAddressData, CreateAssetData, CreateCaseData, CreateNetworkData, CreateReporterData,
    DecodedInstructionData, InstructionData, UpdateAddressData, UpdateAssetData, UpdateCaseData,
    UpdateReporterData, DISCRIMINATOR_SIZE,
};
use crate::{
    client::{events::EventName, result::ClientError},
    HapiCoreSolana,
};

/// Struct representing an Instruction entity from a Solana transaction
pub struct DecodedInstruction {
    /// Sequence index in transaction
    pub id: u8,

    /// HAPI instruction
    pub name: EventName,

    /// Transaction signature hash
    pub tx_hash: String,

    /// The public key of the account containing a program
    pub program_id: String,

    /// Time of transaction block
    pub blocktime: u64,

    /// List of encoded accounts used by the instruction
    pub account_keys: Vec<String>,

    /// Program input data
    pub data: InstructionData,
}

impl HapiCoreSolana {
    pub async fn get_hapi_instructions(&self, hash: &str) -> Result<Vec<DecodedInstruction>> {
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
        tx_accounts: &[String],
        tx_hash: String,
        blocktime: u64,
    ) -> Result<Option<DecodedInstruction>> {
        if instruction.accounts.is_empty() {
            bail!("Empty instruction accounts")
        }

        let instruction_program_id = &tx_accounts[instruction.program_id_index as usize];

        if instruction_program_id == &self.program_id.to_string() {
            let buf = &bs58::decode(&instruction.data).into_vec()?;

            let sighash = &buf[..DISCRIMINATOR_SIZE];

            let name = if let Some(index) = self.hashes.iter().position(|hash| hash == sighash) {
                EventName::from_index(index)?
            } else {
                return Ok(None);
            };

            #[cfg(not(feature = "decode"))]
            let data = InstructionData::Raw(instruction.data.clone());

            #[cfg(feature = "decode")]
            let data = InstructionData::Decoded(decode_instruction_data(
                &name,
                &buf[DISCRIMINATOR_SIZE..],
            )?);

            let account_keys = instruction
                .accounts
                .iter()
                .map(|&index| tx_accounts[index as usize].clone())
                .collect::<Vec<_>>();

            return Ok(Some(DecodedInstruction {
                id,
                tx_hash,
                name,
                program_id: instruction_program_id.to_string(),
                blocktime,
                account_keys,
                data,
            }));
        }

        Ok(None)
    }
}

fn decode_instruction_data(
    hapi_instruction: &EventName,
    data_slice: &[u8],
) -> Result<DecodedInstructionData> {
    let data = match hapi_instruction {
        EventName::Initialize => {
            DecodedInstructionData::CreateNetwork(CreateNetworkData::try_from_slice(data_slice)?)
        }
        EventName::UpdateStakeConfiguration => DecodedInstructionData::UpdateStakeConfiguration(
            StakeConfiguration::try_from_slice(data_slice)?,
        ),
        EventName::UpdateRewardConfiguration => DecodedInstructionData::UpdateRewardConfiguration(
            RewardConfiguration::try_from_slice(data_slice)?,
        ),
        EventName::SetAuthority => DecodedInstructionData::SetAuthority,
        EventName::CreateReporter => {
            DecodedInstructionData::CreateReporter(CreateReporterData::try_from_slice(data_slice)?)
        }
        EventName::UpdateReporter => {
            DecodedInstructionData::UpdateReporter(UpdateReporterData::try_from_slice(data_slice)?)
        }
        EventName::ActivateReporter => DecodedInstructionData::ActivateReporter,
        EventName::DeactivateReporter => DecodedInstructionData::DeactivateReporter,
        EventName::Unstake => DecodedInstructionData::Unstake,
        EventName::CreateCase => {
            DecodedInstructionData::CreateCase(CreateCaseData::try_from_slice(data_slice)?)
        }
        EventName::UpdateCase => {
            DecodedInstructionData::UpdateCase(UpdateCaseData::try_from_slice(data_slice)?)
        }
        EventName::CreateAddress => {
            DecodedInstructionData::CreateAddress(CreateAddressData::try_from_slice(data_slice)?)
        }
        EventName::UpdateAddress => {
            DecodedInstructionData::UpdateAddress(UpdateAddressData::try_from_slice(data_slice)?)
        }
        EventName::ConfirmAddress => {
            DecodedInstructionData::ConfirmAddress(u8::try_from_slice(data_slice)?)
        }
        EventName::CreateAsset => {
            DecodedInstructionData::CreateAsset(CreateAssetData::try_from_slice(data_slice)?)
        }
        EventName::UpdateAsset => {
            DecodedInstructionData::UpdateAsset(UpdateAssetData::try_from_slice(data_slice)?)
        }
        EventName::ConfirmAsset => {
            DecodedInstructionData::ConfirmAsset(u8::try_from_slice(data_slice)?)
        }
    };

    Ok(data)
}

#[cfg(test)]
mod tests {
    use {
        anchor_client::anchor_lang::AnchorSerialize,
        solana_transaction_status::{
            EncodedConfirmedTransactionWithStatusMeta, EncodedTransaction,
            EncodedTransactionWithStatusMeta, UiMessage, UiRawMessage, UiTransaction,
        },
        spl_token::solana_program::message::MessageHeader,
    };

    use crate::{
        client::solana::instruction_data::get_instruction_sighash, HapiCoreNetwork, HapiCoreOptions,
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
            InstructionData::Decoded(data) => match data {
                DecodedInstructionData::CreateNetwork(data) => serialize(name, data),
                DecodedInstructionData::UpdateStakeConfiguration(data) => serialize(name, data),
                DecodedInstructionData::UpdateRewardConfiguration(data) => serialize(name, data),
                DecodedInstructionData::CreateReporter(data) => serialize(name, data),
                DecodedInstructionData::UpdateReporter(data) => serialize(name, data),
                DecodedInstructionData::CreateCase(data) => serialize(name, data),
                DecodedInstructionData::UpdateCase(data) => serialize(name, data),
                DecodedInstructionData::CreateAddress(data) => serialize(name, data),
                DecodedInstructionData::UpdateAddress(data) => serialize(name, data),
                DecodedInstructionData::ConfirmAddress(data) => serialize(name, data),
                DecodedInstructionData::CreateAsset(data) => serialize(name, data),
                DecodedInstructionData::UpdateAsset(data) => serialize(name, data),
                DecodedInstructionData::ConfirmAsset(data) => serialize(name, data),
                _ => get_instruction_sighash(name).to_vec(),
            },
            InstructionData::Raw(data) => serialize(name, data),
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
    #[cfg(not(feature = "decode"))]
    fn get_hapi_transactions() {
        let client = get_cli(None);

        let test_data = [
            "create_network",
            "update_stake_configuration",
            "update_reward_configuration",
            "set_authority",
            "create_reporter",
            "update_reporter",
            "activate_reporter",
            "deactivate_reporter",
            "unstake",
            "create_case",
            "update_case",
            "create_address",
            "update_address",
            "confirm_address",
            "create_asset",
            "update_asset",
            "confirm_asset",
        ]
        .iter()
        .map(|n| (*n, InstructionData::Raw(String::from("Some data"))))
        .collect();

        let instructions = client
            .decode_transaction(create_tx(&test_data))
            .expect("Failed to decode transaction");

        assert_eq!(instructions.len(), test_data.len());
        assert!(instructions
            .iter()
            .all(|instruction| &instruction.program_id == PROGRAM_ID));
    }

    #[test]
    #[cfg(feature = "decode")]
    fn decode_hapi_transactions() {
        let client = get_cli(None);

        let test_data = vec![
            (
                "create_network",
                InstructionData::Decoded(DecodedInstructionData::CreateNetwork(
                    CreateNetworkData::default(),
                )),
            ),
            (
                "update_stake_configuration",
                InstructionData::Decoded(DecodedInstructionData::UpdateStakeConfiguration(
                    StakeConfiguration::default(),
                )),
            ),
            (
                "update_reward_configuration",
                InstructionData::Decoded(DecodedInstructionData::UpdateRewardConfiguration(
                    RewardConfiguration::default(),
                )),
            ),
            (
                "set_authority",
                InstructionData::Decoded(DecodedInstructionData::SetAuthority),
            ),
            (
                "create_reporter",
                InstructionData::Decoded(DecodedInstructionData::CreateReporter(
                    CreateReporterData::default(),
                )),
            ),
            (
                "update_reporter",
                InstructionData::Decoded(DecodedInstructionData::UpdateReporter(
                    UpdateReporterData::default(),
                )),
            ),
            (
                "activate_reporter",
                InstructionData::Decoded(DecodedInstructionData::ActivateReporter),
            ),
            (
                "deactivate_reporter",
                InstructionData::Decoded(DecodedInstructionData::DeactivateReporter),
            ),
            (
                "unstake",
                InstructionData::Decoded(DecodedInstructionData::Unstake),
            ),
            (
                "create_case",
                InstructionData::Decoded(DecodedInstructionData::CreateCase(
                    CreateCaseData::default(),
                )),
            ),
            (
                "update_case",
                InstructionData::Decoded(DecodedInstructionData::UpdateCase(
                    UpdateCaseData::default(),
                )),
            ),
            (
                "create_address",
                InstructionData::Decoded(DecodedInstructionData::CreateAddress(
                    CreateAddressData {
                        address: [1u8; 64],
                        category: Category::Gambling,
                        risk: 5,
                        bump: 255,
                    },
                )),
            ),
            (
                "update_address",
                InstructionData::Decoded(DecodedInstructionData::UpdateAddress(
                    UpdateAddressData::default(),
                )),
            ),
            (
                "confirm_address",
                InstructionData::Decoded(DecodedInstructionData::ConfirmAddress(255)),
            ),
            (
                "create_asset",
                InstructionData::Decoded(DecodedInstructionData::CreateAsset(CreateAssetData {
                    addr: [1u8; 64],
                    asset_id: [1u8; 64],
                    category: Category::ATM,
                    risk_score: 5,
                    bump: 255,
                })),
            ),
            (
                "update_asset",
                InstructionData::Decoded(DecodedInstructionData::UpdateAsset(
                    UpdateAssetData::default(),
                )),
            ),
            (
                "confirm_asset",
                InstructionData::Decoded(DecodedInstructionData::ConfirmAsset(255)),
            ),
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
                InstructionData::Raw(String::from("Some data")),
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
            .decode_transaction(create_tx(&vec![(
                "unstake",
                InstructionData::Raw(String::from("Some data")),
            )]))
            .expect("Failed to decode transaction");

        assert_eq!(instructions.len(), 0);
    }
}
