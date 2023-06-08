use super::{amount::Amount, Uuid};

#[derive(Default, Clone, PartialEq, Debug)]
pub enum ReporterRole {
    #[default]
    Validator = 0,
    Tracer = 1,
    Publisher = 2,
    Authority = 3,
}

#[derive(Default, Clone, PartialEq, Debug)]
pub enum ReporterStatus {
    #[default]
    Inactive = 0,
    Active = 1,
    Unstaking = 2,
}

pub struct CreateReporterInput {}
pub struct UpdateReporterInput {}

#[derive(Default, Clone, Debug)]
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
