use anchor_lang::prelude::*;

/// Reasons the risk check may fail
#[error_code]
pub enum HapiCheckerError {
    #[msg("Unexpected account has been used")]
    UnexpectedAccount,
    #[msg("Account has illegal owner")]
    IllegalOwner,
    #[msg("User account has high risk")]
    HighAccountRisk,
}
