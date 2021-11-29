use anchor_lang::prelude::*;

#[account]
pub struct Case {
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
