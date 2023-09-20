use serde::{de, Deserialize, Serialize};
use std::{
    fmt::{Display, Formatter},
    str::FromStr,
};
use uuid::Uuid;

use crate::client::{amount::Amount, result::ClientError};

#[derive(Default, Clone, PartialEq, Debug)]
pub enum ReporterRole {
    #[default]
    Validator = 0,
    Tracer = 1,
    Publisher = 2,
    Authority = 3,
}

impl Serialize for ReporterRole {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.collect_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for ReporterRole {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        Self::from_str(&s).map_err(de::Error::custom)
    }
}

impl Display for ReporterRole {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ReporterRole::Validator => "Validator",
                ReporterRole::Tracer => "Tracer",
                ReporterRole::Publisher => "Publisher",
                ReporterRole::Authority => "Authority",
            }
        )
    }
}

impl FromStr for ReporterRole {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Validator" | "validator" => Ok(Self::Validator),
            "Tracer" | "tracer" => Ok(Self::Tracer),
            "Publisher" | "publisher" => Ok(Self::Publisher),
            "Authority" | "authority" => Ok(Self::Authority),
            _ => Err(anyhow::anyhow!("invalid reporter role")),
        }
    }
}

impl TryFrom<u8> for ReporterRole {
    type Error = ClientError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Validator),
            1 => Ok(Self::Tracer),
            2 => Ok(Self::Publisher),
            3 => Ok(Self::Authority),
            _ => Err(ClientError::ContractData(format!(
                "invalid reporter role: {value}",
            ))),
        }
    }
}

#[derive(Default, Clone, PartialEq, Debug)]
pub enum ReporterStatus {
    #[default]
    Inactive = 0,
    Active = 1,
    Unstaking = 2,
}

impl Serialize for ReporterStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.collect_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for ReporterStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        Self::from_str(&s).map_err(de::Error::custom)
    }
}

impl Display for ReporterStatus {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ReporterStatus::Inactive => "Inactive",
                ReporterStatus::Active => "Active",
                ReporterStatus::Unstaking => "Unstaking",
            }
        )
    }
}

impl FromStr for ReporterStatus {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Inactive" | "inactive" => Ok(Self::Inactive),
            "Active" | "active" => Ok(Self::Active),
            "Unstaking" | "unstaking" => Ok(Self::Unstaking),
            _ => Err(anyhow::anyhow!("invalid reporter status")),
        }
    }
}

impl TryFrom<u8> for ReporterStatus {
    type Error = ClientError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Inactive),
            1 => Ok(Self::Active),
            2 => Ok(Self::Unstaking),
            _ => Err(ClientError::ContractData(format!(
                "invalid reporter status: {value}",
            ))),
        }
    }
}

pub struct CreateReporterInput {
    pub id: Uuid,
    pub account: String,
    pub role: ReporterRole,
    pub name: String,
    pub url: String,
}

pub struct UpdateReporterInput {
    pub id: Uuid,
    pub account: String,
    pub role: ReporterRole,
    pub name: String,
    pub url: String,
}

#[derive(Default, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Reporter {
    pub id: Uuid,
    pub account: String,
    pub role: ReporterRole,
    pub status: ReporterStatus,
    pub name: String,
    pub url: String,
    pub stake: Amount,
    pub unlock_timestamp: u64,
}
