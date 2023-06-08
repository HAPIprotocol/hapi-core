use super::{Category, Uuid};

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

#[derive(Default, Clone, Debug)]
pub struct Address {
    pub address: String,
    pub case_id: Uuid,
    pub reporter_id: Uuid,
    pub risk: u8,
    pub category: Category,
}
