use anchor_lang::prelude::*;

#[account]
#[derive(Default)]
pub struct Reporter {
    /// Community account, which this reporter belongs to
    pub community: Pubkey,

    /// Seed bump for PDA
    pub bump: u8,

    /// Reporter account status
    pub reporter_status: ReporterStatus,

    /// Reporter's type
    pub reporter_type: ReporterType,

    /// Reporter's wallet account
    pub pubkey: Pubkey,

    /// Short reporter description
    pub name: [u8; 32],

    /// Reporter can unstake at this epoch (0 if unstaking hasn't been requested)
    pub release_epoch: u32,
}

#[derive(Clone, PartialEq, AnchorDeserialize, AnchorSerialize)]
pub enum ReporterStatus {
    /// Reporter is not active and can't interact with the program
    Inactive = 0,

    /// Reporter is not active, but can activate after staking
    OnHold = 1,

    /// Reporter is active and can report
    Active = 2,

    /// Reporter has requested unstaking and can't report
    Unstaking = 3,
}

impl Default for ReporterStatus {
    fn default() -> Self {
        ReporterStatus::Inactive
    }
}

#[derive(Clone, PartialEq, AnchorDeserialize, AnchorSerialize)]
pub enum ReporterType {
    /// Validator - can validate addresses
    Validator = 0,

    /// Tracer - can report and validate addresses
    Tracer = 1,

    /// Full - can report cases and addresses
    Full = 2,

    /// Authority - can report and modify cases and addresses
    Authority = 3,
}

impl Default for ReporterType {
    fn default() -> Self {
        ReporterType::Validator
    }
}
