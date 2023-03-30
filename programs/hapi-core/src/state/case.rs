use crate::utils::DISCRIMINATOR_LENGTH;
use anchor_lang::prelude::*;

#[account]
pub struct Case {
    /// Account version
    pub version: u16,

    /// Community account, which this case belongs to
    pub community: Pubkey,

    /// Seed bump for PDA
    pub bump: u8,

    /// Sequantial case ID
    pub id: u64,

    /// Case reporter's account
    pub reporter: Pubkey,

    /// Case status
    pub status: CaseStatus,

    /// Short case description
    pub name: [u8; 32],
}

impl Case {
    pub const LEN: usize = DISCRIMINATOR_LENGTH + (2 + 32 + 1 + 8 + 32 + 1 + 32);
    pub const VERSION: u16 = 1;
}

#[derive(Clone, PartialEq, AnchorDeserialize, AnchorSerialize)]
pub enum CaseStatus {
    Closed = 0,
    Open = 1,
}

impl Default for CaseStatus {
    fn default() -> Self {
        CaseStatus::Open
    }
}
