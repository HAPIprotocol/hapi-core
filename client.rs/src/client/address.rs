use serde::Serialize;
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

#[derive(Default, Clone, Debug, Serialize)]
pub struct Address {
    pub address: String,
    #[serde(with = "super::uuid")]
    pub case_id: Uuid,
    #[serde(with = "super::uuid")]
    pub reporter_id: Uuid,
    pub risk: u8,
    pub category: Category,
}
