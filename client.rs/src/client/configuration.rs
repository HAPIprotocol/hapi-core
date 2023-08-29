use serde::{Serialize, Deserialize};

use super::amount::Amount;

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct StakeConfiguration {
    pub token: String,
    pub unlock_duration: u64,
    pub validator_stake: Amount,
    pub tracer_stake: Amount,
    pub publisher_stake: Amount,
    pub authority_stake: Amount,
}

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RewardConfiguration {
    pub token: String,
    pub address_confirmation_reward: Amount,
    pub tracer_reward: Amount,
}
