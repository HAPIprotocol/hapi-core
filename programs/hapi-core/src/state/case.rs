use anchor_lang::prelude::*;

#[account]
pub struct Case {
    pub community: Pubkey,
    pub bump: u8,
    pub id: u64,
    pub reporter: Pubkey,
    pub status: CaseStatus,
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
