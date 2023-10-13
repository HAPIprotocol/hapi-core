use anyhow::{bail, Result};
use serde::{de, Deserialize, Serialize};
use std::{
    fmt::{Display, Formatter},
    str::FromStr,
};

#[derive(Debug, PartialEq)]
pub enum EventName {
    // Initialize is equivalent to CreateNetwork in Solana
    Initialize,
    SetAuthority,
    UpdateStakeConfiguration,
    UpdateRewardConfiguration,
    CreateReporter,
    UpdateReporter,
    ActivateReporter,
    DeactivateReporter,
    Unstake,
    CreateCase,
    UpdateCase,
    CreateAddress,
    UpdateAddress,
    ConfirmAddress,
    CreateAsset,
    UpdateAsset,
    ConfirmAsset,
}

impl EventName {
    pub fn from_index(index: usize) -> Result<Self> {
        let instruction = match index {
            0 => EventName::Initialize,
            1 => EventName::SetAuthority,
            2 => EventName::UpdateStakeConfiguration,
            3 => EventName::UpdateRewardConfiguration,
            4 => EventName::CreateReporter,
            5 => EventName::UpdateReporter,
            6 => EventName::ActivateReporter,
            7 => EventName::DeactivateReporter,
            8 => EventName::Unstake,
            9 => EventName::CreateCase,
            10 => EventName::UpdateCase,
            11 => EventName::CreateAddress,
            12 => EventName::UpdateAddress,
            13 => EventName::ConfirmAddress,
            14 => EventName::CreateAsset,
            15 => EventName::UpdateAsset,
            16 => EventName::ConfirmAsset,
            _ => bail!("Invalid instruction index: {}", index),
        };

        Ok(instruction)
    }
}

impl Serialize for EventName {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.collect_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for EventName {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        Self::from_str(&s).map_err(de::Error::custom)
    }
}

impl Display for EventName {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            Self::Initialize => write!(f, "initialize"),
            Self::SetAuthority => write!(f, "set_authority"),
            Self::UpdateStakeConfiguration => write!(f, "update_stake_configuration"),
            Self::UpdateRewardConfiguration => write!(f, "update_reward_configuration"),
            Self::CreateReporter => write!(f, "create_reporter"),
            Self::UpdateReporter => write!(f, "update_reporter"),
            Self::ActivateReporter => write!(f, "activate_reporter"),
            Self::DeactivateReporter => write!(f, "deactivate_reporter"),
            Self::Unstake => write!(f, "unstake"),
            Self::CreateCase => write!(f, "create_case"),
            Self::UpdateCase => write!(f, "update_case"),
            Self::CreateAddress => write!(f, "create_address"),
            Self::UpdateAddress => write!(f, "update_address"),
            Self::ConfirmAddress => write!(f, "confirm_address"),
            Self::CreateAsset => write!(f, "create_asset"),
            Self::UpdateAsset => write!(f, "update_asset"),
            Self::ConfirmAsset => write!(f, "confirm_asset"),
        }
    }
}

// TODO: Separate impl block for all networks to avoid orphan rule
impl FromStr for EventName {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "initialize" | "Initialized" => Ok(Self::Initialize),
            "set_authority" | "AuthorityChanged" => Ok(Self::SetAuthority),
            "update_stake_configuration" | "StakeConfigurationChanged" => {
                Ok(Self::UpdateStakeConfiguration)
            }
            "update_reward_configuration" | "RewardConfigurationChanged" => {
                Ok(Self::UpdateRewardConfiguration)
            }
            "create_reporter" | "ReporterCreated" => Ok(Self::CreateReporter),
            "update_reporter" | "ReporterUpdated" => Ok(Self::UpdateReporter),
            "activate_reporter" | "ReporterActivated" => Ok(Self::ActivateReporter),
            "deactivate_reporter" | "ReporterDeactivated" => Ok(Self::DeactivateReporter),
            "unstake" | "Unstake" | "ReporterStakeWithdrawn" => Ok(Self::Unstake),
            "create_case" | "CaseCreated" => Ok(Self::CreateCase),
            "update_case" | "CaseUpdated" => Ok(Self::UpdateCase),
            "create_address" | "AddressCreated" => Ok(Self::CreateAddress),
            "update_address" | "AddressUpdated" => Ok(Self::UpdateAddress),
            "confirm_address" | "AddressConfirmed" => Ok(Self::ConfirmAddress),
            "create_asset" | "AssetCreated" => Ok(Self::CreateAsset),
            "update_asset" | "AssetUpdated" => Ok(Self::UpdateAsset),
            "confirm_asset" | "AssetConfirmed" => Ok(Self::ConfirmAsset),
            _ => Err(anyhow::anyhow!("invalid event name")),
        }
    }
}
