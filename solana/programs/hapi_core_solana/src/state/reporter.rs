use super::DISCRIMINATOR_LENGTH;
use anchor_lang::prelude::*;

#[account]
pub struct Reporter {
    /// Account version
    pub version: u16,

    /// Seed bump for PDA
    pub bump: u8,

    /// Reporter UUID
    pub id: u128,

    /// Network account
    pub network: Pubkey,

    /// Reporter's wallet account
    pub account: Pubkey,

    /// Short reporter description
    pub name: String,

    /// Reporter's type
    pub role: ReporterRole,

    /// Reporter account status
    pub status: ReporterStatus,

    /// Current deposited stake
    pub stake: u64,

    /// Duration starting from the deactivation moment
    pub unlock_timestamp: u64,

    /// A link to reporter’s public page
    pub url: String,
}

impl Reporter {
    pub const LEN: usize =
        DISCRIMINATOR_LENGTH + (2 + 1 + 16 + 32 + 32 + 128 + 1 + 1 + 8 + 8 + 128);
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

#[derive(Default, Debug, Clone, PartialEq, AnchorDeserialize, AnchorSerialize)]
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
