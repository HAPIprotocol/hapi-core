use anchor_lang::{
    prelude::{AccountInfo, ProgramError},
    AnchorDeserialize,
};

use crate::state::address::Category;

/// Byte index of bump in account data
const OFFSET_BUMP: usize = 136;
/// Risk index of bump in account data
const OFFSET_RISK: usize = 178;
/// Category index of bump in account data
const OFFSET_CATEGORY: usize = 177;

pub struct AddressData {
    pub bump: u8,
    pub risk: u8,
    pub category: Category,
}

impl AddressData {
    pub fn from(account_info: &AccountInfo) -> Result<Self, ProgramError> {
        let data = account_info.try_borrow_data()?;

        let category = Category::try_from_slice(
            &data[OFFSET_CATEGORY..OFFSET_CATEGORY + std::mem::size_of::<Category>()],
        )?;

        Ok(Self {
            bump: data[OFFSET_BUMP],
            risk: data[OFFSET_RISK],
            category,
        })
    }
}
