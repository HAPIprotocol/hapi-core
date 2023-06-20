use serde::Serialize;
use std::str::FromStr;
use uuid::Uuid;

use super::result::ClientError;

#[derive(Default, Clone, PartialEq, Debug, Serialize)]
pub enum CaseStatus {
    #[default]
    Closed = 0,
    Open = 1,
}

impl std::fmt::Display for CaseStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                CaseStatus::Closed => "closed",
                CaseStatus::Open => "open",
            }
        )
    }
}

impl FromStr for CaseStatus {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "closed" => Ok(Self::Closed),
            "open" => Ok(Self::Open),
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

pub struct CreateCaseInput {
    pub id: Uuid,
    pub name: String,
    pub url: String,
}

pub struct UpdateCaseInput {
    pub id: Uuid,
    pub name: String,
    pub url: String,
    pub status: CaseStatus,
}

#[derive(Default, Clone, Debug, Serialize)]
pub struct Case {
    #[serde(with = "super::uuid")]
    pub id: Uuid,
    pub name: String,
    pub url: String,
    pub status: CaseStatus,
}
