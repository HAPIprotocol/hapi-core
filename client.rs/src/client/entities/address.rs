use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::category::Category;

pub struct CreateAddressInput {
    pub address: String,
    pub case_id: Uuid,
    pub risk: u8,
    pub category: Category,
}

pub struct UpdateAddressInput {
    pub address: String,
    pub case_id: Uuid,
    pub risk: u8,
    pub category: Category,
}

pub struct ConfirmAddressInput {
    pub address: String,
}

#[derive(Default, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Address {
    pub address: String,
    pub case_id: Uuid,
    pub reporter_id: Uuid,
    pub risk: u8,
    pub category: Category,
    pub confirmations: u64,
}
