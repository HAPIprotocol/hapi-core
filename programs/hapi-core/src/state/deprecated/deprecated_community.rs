use crate::{error::ErrorCode, state::community::Community};
use {anchor_lang::prelude::*, std::convert::TryInto};

impl Community {
    pub fn from_deprecated(account_data: &mut &[u8]) -> Result<Community> {
        // TODO: current account version must be less than deprecated account version (exept V0)
        let community: Community = match Community::VERSION {
            // Warning! V0 migration can be performed only once
            1 => CommunityV0::try_deserialize_unchecked(account_data)?,
            _ => return Err(ErrorCode::InvalidAccountVersion.into()),
        }
        .try_into()?;

        Ok(community)
    }
}

#[account]
pub struct CommunityV0 {
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

impl TryInto<Community> for CommunityV0 {
    type Error = Error;
    fn try_into(self) -> Result<Community> {
        Ok(Community {
            version: Community::VERSION,
            authority: self.authority,
            cases: self.cases,
            confirmation_threshold: self.confirmation_threshold,
            stake_unlock_epochs: self.stake_unlock_epochs,
            stake_mint: self.stake_mint,
            validator_stake: self.validator_stake,
            tracer_stake: self.tracer_stake,
            full_stake: self.full_stake,
            authority_stake: self.authority_stake,
            appraiser_stake: u64::MAX,
            community_id: 0,
            bump: 0,
        })
    }
}
