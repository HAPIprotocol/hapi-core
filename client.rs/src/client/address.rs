use super::{Category, Uuid};

pub struct CreateAddressInput {}
pub struct UpdateAddressInput {}

#[derive(Default, Clone)]
pub struct Address {
    pub address: String,
    pub case_id: Uuid,
    pub reporter_id: Uuid,
    pub risk: u8,
    pub category: Category,
}
