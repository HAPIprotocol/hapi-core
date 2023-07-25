use super::DISCRIMINATOR_LENGTH;
use anchor_lang::prelude::*;

#[account]
pub struct Confirmation {
    /// Account version
    pub version: u16,

    /// Seed bump for PDA
    pub bump: u8,

    /// Network account
    pub network: Pubkey,

    /// Confirmed account public key
    pub account: Pubkey,

    /// Reporter UUID
    pub reporter_id: u128,
}

impl Confirmation {
    pub const LEN: usize = DISCRIMINATOR_LENGTH + (2 + 1 + 32 + 32 + 16);
    pub const VERSION: u16 = 1;
}
