use {
    anchor_client::anchor_lang::AnchorSerialize,
    solana_transaction_status::{
        EncodedConfirmedTransactionWithStatusMeta, EncodedTransaction,
        EncodedTransactionWithStatusMeta, UiCompiledInstruction, UiMessage, UiRawMessage,
        UiTransaction,
    },
    spl_token::solana_program::message::MessageHeader,
};

use super::instruction_data::{DecodedInstructionData, InstructionData};
use crate::client::solana::instruction_data::get_instruction_sighash;

pub const PROGRAM_ID: &str = "39WzZqJgkK2QuQxV9jeguKRgHE65Q3HywqPwBzdrKn2B";

fn serialize<T: AnchorSerialize>(name: &str, data: T) -> Vec<u8> {
    let mut instruction_data = get_instruction_sighash(name).to_vec();

    let mut inner_data = Vec::new();
    data.serialize(&mut instruction_data)
        .expect("Failed to serialize update address data");

    instruction_data.append(&mut inner_data);
    instruction_data
}

fn create_test_instruction(
    name: &str,
    data: &InstructionData,
    accounts: usize,
) -> UiCompiledInstruction {
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
        accounts: (0..accounts).map(|x| x as u8).collect(),
        data: bs58::encode(instruction_data).into_string(),
        stack_height: None,
    }
}

pub fn create_test_tx(
    data: &Vec<(&str, InstructionData)>,
    signature: String,
    account_keys: Vec<String>,
) -> EncodedConfirmedTransactionWithStatusMeta {
    let instructions = data
        .iter()
        .map(|(name, data)| create_test_instruction(name, data, account_keys.len()))
        .collect();

    EncodedConfirmedTransactionWithStatusMeta {
        slot: 123,
        transaction: EncodedTransactionWithStatusMeta {
            transaction: EncodedTransaction::Json(UiTransaction {
                signatures: vec![signature],
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
