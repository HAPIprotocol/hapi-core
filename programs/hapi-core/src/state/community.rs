use anchor_lang::prelude::*;

#[account]
pub struct Community {
    pub authority: Pubkey,
    pub case_count: u64,
}
