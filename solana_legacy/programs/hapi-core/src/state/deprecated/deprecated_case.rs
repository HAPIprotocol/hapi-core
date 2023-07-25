use crate::{
    error::ErrorCode,
    state::case::{Case, CaseStatus},
};
use {anchor_lang::prelude::*, std::convert::TryInto};

impl Case {
    pub fn from_deprecated(account_data: &mut &[u8]) -> Result<Case> {
        // TODO: current account version must be less than deprecated account version (exept V0)
        let reward: Case = match Case::VERSION {
            // Warning! V0 migration can be performed only once
            1 => CaseV0::try_deserialize_unchecked(account_data)?,
            _ => return Err(ErrorCode::InvalidAccountVersion.into()),
        }
        .try_into()?;

        Ok(reward)
    }
}

#[account]
pub struct CaseV0 {
    pub community: Pubkey,
    pub bump: u8,
    pub id: u64,
    pub reporter: Pubkey,
    pub status: CaseStatus,
    pub name: [u8; 32],
}

impl TryInto<Case> for CaseV0 {
    type Error = Error;
    fn try_into(self) -> Result<Case> {
        Ok(Case {
            version: Case::VERSION,
            community: self.community,
            bump: self.bump,
            id: self.id,
            reporter: self.reporter,
            status: self.status,
            name: self.name,
        })
    }
}
