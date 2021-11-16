use anchor_lang::prelude::*;

declare_id!("YSZQu65s65fUchUSBMtTDp7MUuTyPEDYE6pKqGeZjCE");

#[program]
pub mod hapi_reward {
    use super::*;

    pub fn initialize_network(ctx: Context<InitializeNetwork>, base_reward: u64) -> ProgramResult {
        let network_profile = &mut ctx.accounts.network_profile;
        network_profile.base_reward = base_reward;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeNetwork<'info> {
    #[account(init, payer = authority, space = 8 + 32 + 8)]
    pub network_profile: Account<'info, NetworkProfile>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct NetworkProfile {
    pub network_account: Pubkey,
    pub base_reward: u64,
}
