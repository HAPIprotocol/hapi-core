use anchor_lang::prelude::*;

#[account]
pub struct Community {
    pub authority: Pubkey,
    pub cases: u64,
}
