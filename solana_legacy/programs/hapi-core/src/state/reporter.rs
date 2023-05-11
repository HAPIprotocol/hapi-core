use crate::utils::DISCRIMINATOR_LENGTH;
use anchor_lang::prelude::*;

#[account]
#[derive(Default)]
pub struct Reporter {
    /// Account version
    pub version: u16,

    /// Community account, which this reporter belongs to
    pub community: Pubkey,

    /// Seed bump for PDA
    pub bump: u8,

    /// If this is true, reporter can't interact with the contract
    pub is_frozen: bool,

    /// Reporter account status
    pub status: ReporterStatus,

    /// Reporter's type
    pub role: ReporterRole,

    /// Reporter's wallet account
    pub pubkey: Pubkey,

    /// Short reporter description
    pub name: [u8; 32],

    /// Current deposited stake
    pub stake: u64,

    /// Reporter can unstake at this epoch (0 if unstaking hasn't been requested)
    pub unlock_epoch: u64,
}

impl Reporter {
    pub const LEN: usize = DISCRIMINATOR_LENGTH + (2 + 32 + 1 + 1 + 1 + 1 + 32 + 32 + 8 + 8);
    pub const VERSION: u16 = 1;
}

#[account]
#[derive(Default, Debug)]
pub struct ReporterReward {
    /// Account version
    pub version: u16,

    /// Reporter account to keep reward counter for
    pub reporter: Pubkey,

    /// Network that has the reward associated with
    pub network: Pubkey,

    /// Seed bump for PDA
    pub bump: u8,

    /// Number of unclaimed address report rewards
    pub address_tracer_counter: u64,

    /// Number of unclaimed address confirmation rewards
    pub address_confirmation_counter: u64,

    /// Number of unclaimed asset report rewards
    pub asset_tracer_counter: u64,

    /// Number of unclaimed asset confirmation rewards
    pub asset_confirmation_counter: u64,
}

impl ReporterReward {
    pub const LEN: usize = DISCRIMINATOR_LENGTH + (2 + 32 + 32 + 1 + 8 + 8 + 8 + 8);
    pub const VERSION: u16 = 1;
}

#[derive(Default, Clone, PartialEq, AnchorDeserialize, AnchorSerialize)]
pub enum ReporterStatus {
    /// Reporter is not active, but can activate after staking
    #[default]
    Inactive,

    /// Reporter is active and can report
    Active,

    /// Reporter has requested unstaking and can't report
    Unstaking,
}

#[derive(Default, Clone, PartialEq, AnchorDeserialize, AnchorSerialize)]
pub enum ReporterRole {
    /// Validator - can validate addresses
    #[default]
    Validator = 0,

    /// Tracer - can report and validate addresses
    Tracer = 1,

    /// Publisher - can report cases and addresses
    Publisher = 2,

    /// Authority - can report and modify cases and addresses
    Authority = 3,

    /// Appraiser - can update replication price
    Appraiser = 4,
}
