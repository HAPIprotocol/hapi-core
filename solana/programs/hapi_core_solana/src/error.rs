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
    #[msg("Program data account is absent")]
    AbsentProgramData,
}

pub fn print_error(error: ErrorCode) -> Result<()> {
    msg!("Error: {}", error);
    Err(error.into())
}
