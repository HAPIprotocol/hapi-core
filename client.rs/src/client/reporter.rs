use serde::Serialize;
use std::str::FromStr;

use super::{amount::Amount, result::ClientError, Uuid};

#[derive(Default, Clone, PartialEq, Debug, Serialize)]
pub enum ReporterRole {
    #[default]
    Validator = 0,
    Tracer = 1,
    Publisher = 2,
    Authority = 3,
}

impl std::fmt::Display for ReporterRole {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ReporterRole::Validator => "validator",
                ReporterRole::Tracer => "tracer",
                ReporterRole::Publisher => "publisher",
                ReporterRole::Authority => "authority",
            }
        )
    }
}

impl FromStr for ReporterRole {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "validator" => Ok(Self::Validator),
            "tracer" => Ok(Self::Tracer),
            "publisher" => Ok(Self::Publisher),
            "authority" => Ok(Self::Authority),
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

#[derive(Default, Clone, PartialEq, Debug, Serialize)]
pub enum ReporterStatus {
    #[default]
    Inactive = 0,
    Active = 1,
    Unstaking = 2,
}

impl std::fmt::Display for ReporterStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ReporterStatus::Inactive => "inactive",
                ReporterStatus::Active => "active",
                ReporterStatus::Unstaking => "unstaking",
            }
        )
    }
}

impl FromStr for ReporterStatus {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "inactive" => Ok(Self::Inactive),
            "active" => Ok(Self::Active),
            "unstaking" => Ok(Self::Unstaking),
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
    pub id: u128,
    pub account: String,
    pub role: ReporterRole,
    pub name: String,
    pub url: String,
}

pub struct UpdateReporterInput {
    pub id: u128,
    pub account: String,
    pub role: ReporterRole,
    pub name: String,
    pub url: String,
}

#[derive(Default, Clone, Debug, Serialize)]
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
