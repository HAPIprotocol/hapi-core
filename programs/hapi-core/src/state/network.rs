use anchor_lang::prelude::*;

#[account]
pub struct Network {
    pub community: Pubkey,
    pub bump: u8,
    pub name: [u8; 32],
}
