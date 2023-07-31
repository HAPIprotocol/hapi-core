use serde::{de, Deserialize, Serialize};
use std::{
    fmt::{Display, Formatter},
    str::FromStr,
};
use uuid::Uuid;

use hapi_core::{
    client::{
        case::CaseStatus,
        category::Category,
        reporter::{ReporterRole, ReporterStatus},
    },
    Amount,
};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct PushPayload {
    event: PushEvent,
    mode: PushMode,
    data: PushData,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct PushEvent {
    name: PushEventName,
    tx_hash: String,
    tx_index: u64,
    timestamp: String,
}

#[derive(Debug, PartialEq)]
enum PushEventName {
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

impl Serialize for PushEventName {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.collect_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for PushEventName {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        Self::from_str(&s).map_err(de::Error::custom)
    }
}

impl Display for PushEventName {
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

impl FromStr for PushEventName {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "initialize" => Ok(Self::Initialize),
            "set_authority" => Ok(Self::SetAuthority),
            "update_stake_configuration" => Ok(Self::UpdateStakeConfiguration),
            "update_reward_configuration" => Ok(Self::UpdateRewardConfiguration),
            "create_reporter" => Ok(Self::CreateReporter),
            "update_reporter" => Ok(Self::UpdateReporter),
            "activate_reporter" => Ok(Self::ActivateReporter),
            "deactivate_reporter" => Ok(Self::DeactivateReporter),
            "unstake" => Ok(Self::Unstake),
            "create_case" => Ok(Self::CreateCase),
            "update_case" => Ok(Self::UpdateCase),
            "create_address" => Ok(Self::CreateAddress),
            "update_address" => Ok(Self::UpdateAddress),
            "confirm_address" => Ok(Self::ConfirmAddress),
            "create_asset" => Ok(Self::CreateAsset),
            "update_asset" => Ok(Self::UpdateAsset),
            "confirm_asset" => Ok(Self::ConfirmAsset),
            _ => Err(anyhow::anyhow!("invalid event name")),
        }
    }
}

#[derive(Debug, PartialEq)]
enum PushMode {
    Create,
    Update,
}

impl Serialize for PushMode {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.collect_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for PushMode {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        Self::from_str(&s).map_err(de::Error::custom)
    }
}

impl Display for PushMode {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            Self::Create => write!(f, "create"),
            Self::Update => write!(f, "update"),
        }
    }
}

impl FromStr for PushMode {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "create" => Ok(Self::Create),
            "update" => Ok(Self::Update),
            _ => Err(anyhow::anyhow!("invalid push mode")),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
enum PushData {
    Address {
        address: String,
        category: Category,
        risk_score: u8,
        case_id: Uuid,
        reporter_id: Uuid,
        confirmations: u64,
    },
    Asset {
        address: String,
        asset_id: String,
        category: Category,
        risk_score: u8,
        case_id: Uuid,
        reporter_id: Uuid,
        confirmations: u64,
    },
    Case {
        id: Uuid,
        name: String,
        url: String,
        status: CaseStatus,
    },
    Reporter {
        id: Uuid,
        account: String,
        role: ReporterRole,
        status: ReporterStatus,
        name: String,
        url: String,
        stake: Amount,
        unlock_timestamp: u64,
    },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_push_payload_serialization() {
        // Create a sample PushPayload
        let payload = PushPayload {
            event: PushEvent {
                name: PushEventName::CreateAddress,
                tx_hash: "acf0734ab380f3964e1f23b1fd4f5a5125250208ec17ff11c9999451c138949f"
                    .to_string(),
                tx_index: 0,
                timestamp: "2022-01-01T00:00:00Z".to_string(),
            },
            mode: PushMode::Create,
            data: PushData::Address {
                address: "0x922ffdfcb57de5dd6f641f275e98b684ce5576a3".to_string(),
                case_id: uuid::uuid!("de1659f2-b802-49ee-98dd-6e4ce0453067"),
                reporter_id: uuid::uuid!("1466cf4f-1d71-4153-b9ad-4a9c1b48101e"),
                category: Category::None,
                risk_score: 0,
                confirmations: 0,
            },
        };

        // Serialize the PushPayload to JSON
        let json = serde_json::to_string(&payload).unwrap();

        assert_eq!(
            json,
            r#"{"event":{"name":"create_address","tx_hash":"acf0734ab380f3964e1f23b1fd4f5a5125250208ec17ff11c9999451c138949f","tx_index":0,"timestamp":"2022-01-01T00:00:00Z"},"entity":"address","mode":"create","data":{"Address":{"address":"0x922ffdfcb57de5dd6f641f275e98b684ce5576a3","category":"None","risk_score":0,"case_id":"de1659f2-b802-49ee-98dd-6e4ce0453067","reporter_id":"1466cf4f-1d71-4153-b9ad-4a9c1b48101e","confirmations":0}}}"#
        );

        // Deserialize the JSON back into a PushPayload
        let deserialized_payload: PushPayload = serde_json::from_str(&json).unwrap();

        // Ensure that the deserialized PushPayload matches the original
        assert_eq!(payload, deserialized_payload);
    }
}
