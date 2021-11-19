use anchor_lang::prelude::*;

#[account]
pub struct Network {
    pub bump: u8,
    pub name: [u8; 32],
}
