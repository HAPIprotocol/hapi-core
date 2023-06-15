use serde::Serialize;
use uuid::Uuid;

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

pub struct CreateCaseInput {}

pub struct UpdateCaseInput {}

#[derive(Default, Clone, Debug, Serialize)]
pub struct Case {
    #[serde(with = "super::uuid")]
    pub id: Uuid,
    pub name: String,
    pub url: String,
    pub status: CaseStatus,
}
