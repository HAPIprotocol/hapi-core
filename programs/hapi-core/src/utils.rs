use crate::error::{print_error, ErrorCode};
use {anchor_lang::prelude::*, std::io::Write};

/// Anchor discriminator length
pub const DISCRIMINATOR_LENGTH: usize = 8;
/// Account reserve space
pub const ACCOUNT_RESERVE_SPACE: usize = 32;

fn realloc_and_rent<'info>(
    account: &AccountInfo<'info>,
    payer: &Signer<'info>,
    rent: &Sysvar<'info, Rent>,
    len: usize,
) -> Result<()> {
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
    )?;

    Ok(())
}

pub fn migrate<'info, Acc: AccountSerialize>(
    account_data: Acc,
    account: &AccountInfo<'info>,
    payer: &Signer<'info>,
    rent: &Sysvar<'info, Rent>,
    len: usize,
) -> Result<()> {
    let mut buffer: Vec<u8> = Vec::new();
    account_data.try_serialize(&mut buffer)?;

    if buffer.len() != len {
        return print_error(ErrorCode::UnexpectedLength);
    }

    realloc_and_rent(account, payer, rent, len + ACCOUNT_RESERVE_SPACE)?;
    account.try_borrow_mut_data()?.write_all(&buffer)?;

    Ok(())
}
