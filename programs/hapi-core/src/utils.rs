use anchor_lang::{prelude::*, solana_program::entrypoint::ProgramResult};
pub const DISCRIMINATOR_LENGTH: usize = 8;

pub fn realloc_and_rent<'info>(
    account: &AccountInfo<'info>,
    payer: &Signer<'info>,
    rent: &Sysvar<'info, Rent>,
    len: usize,
) -> ProgramResult {
    // Realloc
    account.realloc(len, false)?;

    let balance = account.lamports();
    if rent.is_exempt(balance, len) {
        return Ok(());
    }

    // Transfer some lamports
    let min_balance = rent.minimum_balance(len);
    if balance.ge(&min_balance) {
        return Ok(());
    }

    let ix = anchor_lang::solana_program::system_instruction::transfer(
        &payer.key(),
        &account.key(),
        min_balance - balance,
    );

    anchor_lang::solana_program::program::invoke(
        &ix,
        &[payer.to_account_info(), account.to_account_info()],
    )
}
