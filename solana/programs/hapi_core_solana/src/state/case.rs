use super::DISCRIMINATOR_LENGTH;
use anchor_lang::prelude::*;

#[account]
pub struct Case {
    /// Account version
    pub version: u16,

    /// Seed bump for PDA
    pub bump: u8,

    /// Case UUID
    pub id: u128,

    /// Network account
    pub network: Pubkey,

    /// Short case description
    pub name: String,

    /// Case reporter's account
    pub reporter: Pubkey,

    /// Case status
    pub status: CaseStatus,

    /// A link to publicly available case documentation
    pub url: String,
}

impl Case {
    pub const LEN: usize = DISCRIMINATOR_LENGTH + (2 + 1 + 16 + 32 + 128 + 32 + 1 + 128);
    pub const VERSION: u16 = 1;
}

#[derive(Default, Clone, PartialEq, AnchorDeserialize, AnchorSerialize)]
pub enum CaseStatus {
    /// Investigations over this case are finished
    Closed = 0,

    /// The case is on-going
    #[default]
    Open = 1,
}
