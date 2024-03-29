use serde::{de, Deserialize, Serialize};
use std::str::FromStr;
use uuid::Uuid;

use crate::client::result::ClientError;

#[derive(Default, Clone, PartialEq, Debug)]
pub enum CaseStatus {
    #[default]
    Closed = 0,
    Open = 1,
}

impl Serialize for CaseStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.collect_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for CaseStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        Self::from_str(&s).map_err(de::Error::custom)
    }
}

impl std::fmt::Display for CaseStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                CaseStatus::Closed => "Closed",
                CaseStatus::Open => "Open",
            }
        )
    }
}

impl FromStr for CaseStatus {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Closed" | "closed" => Ok(Self::Closed),
            "Open" | "open" => Ok(Self::Open),
            _ => Err(anyhow::anyhow!("invalid case status")),
        }
    }
}

impl TryFrom<u8> for CaseStatus {
    type Error = ClientError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Closed),
            1 => Ok(Self::Open),
            _ => Err(ClientError::ContractData(format!(
                "invalid case status: {value}",
            ))),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreateCaseInput {
    pub id: Uuid,
    pub name: String,
    pub url: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UpdateCaseInput {
    pub id: Uuid,
    pub name: String,
    pub url: String,
    pub status: CaseStatus,
}

#[derive(Default, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Case {
    pub id: Uuid,
    pub name: String,
    pub url: String,
    pub status: CaseStatus,
    pub reporter_id: Uuid,
}
