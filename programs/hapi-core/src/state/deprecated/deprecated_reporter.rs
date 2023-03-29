use crate::{error::ErrorCode, state::reporter::ReporterReward};
use {anchor_lang::prelude::*, std::convert::TryInto};

impl ReporterReward {
    pub fn from_deprecated(version: u8, account_data: &mut &[u8]) -> Result<ReporterReward> {
        let reward: ReporterReward = match version {
            1 => ReporterRewardV1::try_deserialize_unchecked(account_data)?,
            _ => return Err(ErrorCode::InvalidAccountVersion.into()),
        }
        .try_into()?;

        Ok(reward)
    }
}

#[account(zero_copy)]
#[derive(Default, Debug)]
pub struct ReporterRewardV1 {
    pub reporter: Pubkey,
    pub network: Pubkey,
    pub bump: u8,
    pub address_tracer_counter: u64,
    pub address_confirmation_counter: u64,
    pub asset_tracer_counter: u64,
    pub asset_confirmation_counter: u64,
}

impl TryInto<ReporterReward> for ReporterRewardV1 {
    type Error = Error;
    fn try_into(self) -> Result<ReporterReward> {
        Ok(ReporterReward {
            reporter: self.reporter,
            network: self.network,
            bump: self.bump,
            address_tracer_counter: self.address_tracer_counter,
            address_confirmation_counter: self.address_confirmation_counter,
            asset_tracer_counter: self.asset_tracer_counter,
            asset_confirmation_counter: self.asset_confirmation_counter,
        })
    }
}
