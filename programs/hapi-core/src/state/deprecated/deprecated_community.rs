use crate::{error::ErrorCode, state::community::Community};
use {anchor_lang::prelude::*, std::convert::TryInto};

impl Community {
    pub fn from_deprecated(version: u8, account_data: &mut &[u8]) -> Result<Community> {
        let community: Community = match version {
            1 => CommunityV1::try_deserialize_unchecked(account_data)?,
            _ => return Err(ErrorCode::InvalidAccountVersion.into()),
        }
        .try_into()?;

        Ok(community)
    }
}

#[account]
pub struct CommunityV1 {
    pub authority: Pubkey,
    pub cases: u64,
    pub confirmation_threshold: u8,
    pub stake_unlock_epochs: u64,
    pub stake_mint: Pubkey,
    pub token_signer: Pubkey,
    pub token_signer_bump: u8,
    pub token_account: Pubkey,
    pub validator_stake: u64,
    pub tracer_stake: u64,
    pub full_stake: u64,
    pub authority_stake: u64,
}

impl TryInto<Community> for CommunityV1 {
    type Error = Error;
    fn try_into(self) -> Result<Community> {
        Ok(Community {
            authority: self.authority,
            cases: self.cases,
            confirmation_threshold: self.confirmation_threshold,
            stake_unlock_epochs: self.stake_unlock_epochs,
            stake_mint: self.stake_mint,
            token_signer: self.token_signer,
            token_signer_bump: self.token_signer_bump,
            token_account: self.token_account,
            validator_stake: self.validator_stake,
            tracer_stake: self.tracer_stake,
            full_stake: self.full_stake,
            authority_stake: self.authority_stake,
            treasury_token_account: Pubkey::default(),
            appraiser_stake: u64::MAX,
        })
    }
}
