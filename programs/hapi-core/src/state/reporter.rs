use anchor_lang::prelude::*;

#[account]
#[derive(Default)]
pub struct Reporter {
    pub bump: u8,
    pub reporter_type: ReporterType,
    pub pubkey: Pubkey,
    pub name: [u8; 32],
}

#[repr(u32)]
#[derive(Clone, AnchorDeserialize, AnchorSerialize)]
pub enum ReporterType {
    /// Inactive reporter
    Inactive,

    /// Tracer - can report addresses
    Tracer,

    /// Full - can report cases and addresses
    Full,

    /// Authority - can modify cases and addresses
    Authority,
}

impl Default for ReporterType {
    fn default() -> Self {
        ReporterType::Inactive
    }
}
