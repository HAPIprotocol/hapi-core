use crate::{
    error::ErrorCode,
    state::reporter::{Reporter, ReporterRole, ReporterStatus},
};
use {anchor_lang::prelude::*, std::convert::TryInto};

impl Reporter {
    pub fn from_deprecated(account_data: &mut &[u8]) -> Result<Reporter> {
        // TODO: current account version must be less than deprecated account version (exept V0)
        let reward: Reporter = match Reporter::VERSION {
            // Warning! V0 migration can be performed only once
            1 => ReporterV0::try_deserialize_unchecked(account_data)?,
            _ => return Err(ErrorCode::InvalidAccountVersion.into()),
        }
        .try_into()?;

        Ok(reward)
    }
}

#[account]
#[derive(Default)]
pub struct ReporterV0 {
    pub community: Pubkey,
    pub bump: u8,
    pub is_frozen: bool,
    pub status: ReporterStatus,
    pub role: ReporterRole,
    pub pubkey: Pubkey,
    pub name: [u8; 32],
    pub stake: u64,
    pub unlock_epoch: u64,
}

impl TryInto<Reporter> for ReporterV0 {
    type Error = Error;
    fn try_into(self) -> Result<Reporter> {
        Ok(Reporter {
            version: Reporter::VERSION,
            community: self.community,
            bump: self.bump,
            is_frozen: self.is_frozen,
            status: self.status,
            role: self.role,
            pubkey: self.pubkey,
            name: self.name,
            stake: self.stake,
            unlock_epoch: self.unlock_epoch,
        })
    }
}
