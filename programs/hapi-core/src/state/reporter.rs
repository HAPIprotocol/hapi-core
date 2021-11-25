use anchor_lang::prelude::*;

#[account]
#[derive(Default)]
pub struct Reporter {
    pub community: Pubkey,
    pub bump: u8,
    pub reporter_type: ReporterType,
    pub pubkey: Pubkey,
    pub name: [u8; 32],
}

#[derive(Clone, PartialEq, AnchorDeserialize, AnchorSerialize)]
pub enum ReporterType {
    /// Inactive reporter
    Inactive = 0,

    /// Tracer - can report addresses
    Tracer = 1,

    /// Full - can report cases and addresses
    Full = 2,

    /// Authority - can modify cases and addresses
    Authority = 3,
}

impl Default for ReporterType {
    fn default() -> Self {
        ReporterType::Inactive
    }
}
