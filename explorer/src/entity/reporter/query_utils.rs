use async_graphql::{Enum, InputObject};

use super::model::Column;
use crate::entity::types::{NetworkName, ReporterRole, ReporterStatus};

/// Conditions to filter address listings by
#[derive(Clone, Eq, PartialEq, InputObject, Debug)]
pub struct ReporterFilter {
    pub network: Option<NetworkName>,
    pub account: Option<String>,
    pub role: Option<ReporterRole>,
    pub status: Option<ReporterStatus>,
    pub name: Option<String>,
    pub url: Option<String>,
}

/// Available ordering values for asset
#[derive(Enum, Copy, Clone, Eq, PartialEq, Default, Debug)]
pub enum ReporterCondition {
    #[default]
    /// Order by network
    Network,
    /// Order by reporter id
    ReporterId,
    /// Order by reporter account
    Account,
    /// Order by reporter role
    Role,
    /// Order by reporter status
    Status,
    /// Order by name
    Name,
    /// Order by url
    Url,
    /// Order by stake
    Stake,
    /// Order by unlock timestamp
    UnlockTimestamp,
    /// Order by the time when entity was created
    CreatedAt,
    /// Order by the time when entity was updated
    UpdatedAt,
}

impl From<ReporterCondition> for Column {
    fn from(condition: ReporterCondition) -> Self {
        match condition {
            ReporterCondition::Network => Column::Network,
            ReporterCondition::ReporterId => Column::ReporterId,
            ReporterCondition::Account => Column::Account,
            ReporterCondition::Role => Column::Role,
            ReporterCondition::Status => Column::Status,
            ReporterCondition::Name => Column::Name,
            ReporterCondition::Url => Column::Url,
            ReporterCondition::Stake => Column::Stake,
            ReporterCondition::UnlockTimestamp => Column::UnlockTimestamp,
            ReporterCondition::CreatedAt => Column::CreatedAt,
            ReporterCondition::UpdatedAt => Column::UpdatedAt,
        }
    }
}
