use ethers::utils::to_checksum;
use uuid::Uuid;

use crate::client::{
    entities::reporter::{Reporter, ReporterRole},
    result::{ClientError, Result},
};
use hapi_core_solana::{Reporter as SolanaReporter, ReporterRole as SolanaReporterRole};

impl From<ReporterRole> for SolanaReporterRole {
    fn from(value: ReporterRole) -> Self {
        match value {
            ReporterRole::Validator => SolanaReporterRole::Validator,
            ReporterRole::Tracer => SolanaReporterRole::Tracer,
            ReporterRole::Publisher => SolanaReporterRole::Publisher,
            ReporterRole::Authority => SolanaReporterRole::Authority,
            _ => panic!("Invalid value for ReporterRole"),
        }
    }
}

impl TryFrom<SolanaReporter> for Reporter {
    type Error = ClientError;

    fn try_from(reporter: SolanaReporter) -> Result<Self> {
        Ok(Reporter {
            id: Uuid::from_u128(reporter.id),
            account: reporter.account.to_string(),
            role: (reporter.role as u8).try_into()?,
            status: (reporter.status as u8).try_into()?,
            name: reporter.name.to_string(),
            url: reporter.url.to_string(),
            stake: reporter.stake.into(),
            unlock_timestamp: reporter.unlock_timestamp,
        })
    }
}

// use crate::{
//     client::{
//         configuration::{RewardConfiguration, StakeConfiguration},
//         entities::{address::Address, asset::Asset, case::Case, reporter::Reporter},
//         result::{ClientError, Result},
//     },
//     Amount,
// };

// use super::client::hapi_core_contract;

// impl From<hapi_core_contract::StakeConfiguration> for StakeConfiguration {
//     fn from(config: hapi_core_contract::StakeConfiguration) -> Self {
//         StakeConfiguration {
//             token: to_checksum(&config.token, None),
//             unlock_duration: config.unlock_duration.as_u64(),
//             validator_stake: config.validator_stake.into(),
//             tracer_stake: config.tracer_stake.into(),
//             publisher_stake: config.publisher_stake.into(),
//             authority_stake: config.authority_stake.into(),
//         }
//     }
// }

// impl From<hapi_core_contract::RewardConfiguration> for RewardConfiguration {
//     fn from(config: hapi_core_contract::RewardConfiguration) -> Self {
//         // TODO: add asset rewards
//         RewardConfiguration {
//             token: to_checksum(&config.token, None),
//             address_confirmation_reward: config.address_confirmation_reward.into(),
//             address_tracer_reward: config.tracer_reward.into(),
//             asset_confirmation_reward: Amount::default(),
//             asset_tracer_reward: Amount::default(),
//         }
//     }
// }

// impl TryFrom<hapi_core_contract::Reporter> for Reporter {
//     type Error = ClientError;

//     fn try_from(reporter: hapi_core_contract::Reporter) -> Result<Self> {
//         Ok(Reporter {
//             id: Uuid::from_u128(reporter.id),
//             account: to_checksum(&reporter.account, None),
//             name: reporter.name.to_string(),
//             url: reporter.url.to_string(),
//             role: reporter.role.try_into()?,
//             status: reporter.status.try_into()?,
//             stake: reporter.stake.into(),
//             unlock_timestamp: reporter.unlock_timestamp.as_u64(),
//         })
//     }
// }

// impl TryFrom<hapi_core_contract::Case> for Case {
//     type Error = ClientError;

//     fn try_from(case: hapi_core_contract::Case) -> Result<Self> {
//         Ok(Case {
//             id: Uuid::from_u128(case.id),
//             name: case.name.to_string(),
//             url: case.url.to_string(),
//             status: case.status.try_into()?,
//         })
//     }
// }

// impl TryFrom<hapi_core_contract::Address> for Address {
//     type Error = ClientError;

//     fn try_from(address: hapi_core_contract::Address) -> Result<Self> {
//         Ok(Address {
//             address: to_checksum(&address.addr, None),
//             case_id: Uuid::from_u128(address.case_id),
//             reporter_id: Uuid::from_u128(address.reporter_id),
//             risk: address.risk,
//             category: address.category.try_into()?,
//         })
//     }
// }

// impl TryFrom<hapi_core_contract::Asset> for Asset {
//     type Error = ClientError;

//     fn try_from(asset: hapi_core_contract::Asset) -> Result<Self> {
//         Ok(Asset {
//             address: to_checksum(&asset.addr, None),
//             asset_id: asset.asset_id.into(),
//             case_id: Uuid::from_u128(asset.case_id),
//             reporter_id: Uuid::from_u128(asset.reporter_id),
//             risk: asset.risk,
//             category: asset.category.try_into()?,
//         })
//     }
// }
