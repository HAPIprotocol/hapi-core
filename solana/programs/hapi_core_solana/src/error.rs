use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Invalid token account")]
    InvalidToken,
    #[msg("Authority mismatched")]
    AuthorityMismatch,
    #[msg("Account has illegal owner")]
    IllegalOwner,
    #[msg("Invalid program data account")]
    InvalidProgramData,
    #[msg("Invalid program account")]
    InvalidProgramAccount,
    #[msg("Invalid reporter account")]
    InvalidReporter,
    #[msg("Invalid reporter status")]
    InvalidReporterStatus,
    #[msg("Reporter account is not active")]
    InactiveReporter,
    #[msg("This reporter is frozen")]
    FrozenReporter,
    #[msg("Release epoch is in future")]
    ReleaseEpochInFuture,
    #[msg("Mint has already been updated")]
    UpdatedMint,
    #[msg("Account is not authorized to perform this action")]
    Unauthorized,
    #[msg("Invalid UUID")]
    InvalidUUID,
    #[msg("Invalid Data")]
    InvalidData,
    #[msg("Case closed")]
    CaseClosed,
    #[msg("Case mismatched")]
    CaseMismatch,
    #[msg("Risk score must be in 0..10 range")]
    RiskOutOfRange,
}

pub fn print_error(error: ErrorCode) -> Result<()> {
    msg!("Error: {}", error);
    Err(error.into())
}
