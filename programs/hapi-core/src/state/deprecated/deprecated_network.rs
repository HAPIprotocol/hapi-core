use crate::{
    error::ErrorCode,
    state::network::{Network, NetworkSchema},
};
use {anchor_lang::prelude::*, std::convert::TryInto};

impl Network {
    pub fn from_deprecated(account_data: &mut &[u8]) -> Result<Network> {
        // TODO: current account version must be less than deprecated account version (exept V0)
        let network: Network = match Network::VERSION {
            // Warning! V0 migration can be performed only once
            1 => NetworkV0::try_deserialize_unchecked(account_data)?,
            _ => return Err(ErrorCode::InvalidAccountVersion.into()),
        }
        .try_into()?;

        Ok(network)
    }
}

#[account]
pub struct NetworkV0 {
    pub community: Pubkey,
    pub bump: u8,
    pub name: [u8; 32],
    pub schema: NetworkSchema,
    pub reward_mint: Pubkey,
    pub reward_signer: Pubkey,
    pub reward_signer_bump: u8,
    pub address_tracer_reward: u64,
    pub address_confirmation_reward: u64,
    pub asset_tracer_reward: u64,
    pub asset_confirmation_reward: u64,
}

impl TryInto<Network> for NetworkV0 {
    type Error = Error;
    fn try_into(self) -> Result<Network> {
        Ok(Network {
            version: Network::VERSION,
            community: self.community,
            bump: self.bump,
            name: self.name,
            schema: self.schema,
            reward_mint: self.reward_mint,
            reward_signer: self.reward_signer,
            reward_signer_bump: self.reward_signer_bump,
            address_tracer_reward: self.address_tracer_reward,
            address_confirmation_reward: self.address_confirmation_reward,
            asset_tracer_reward: self.asset_tracer_reward,
            asset_confirmation_reward: self.asset_confirmation_reward,
            replication_price: 0,
        })
    }
}
