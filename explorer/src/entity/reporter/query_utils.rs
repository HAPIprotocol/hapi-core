use async_graphql::{Enum, InputObject};

use super::model::Column;
use crate::entity::types::{ReporterRole, ReporterStatus};

/// Conditions to filter address listings by
#[derive(Clone, Eq, PartialEq, InputObject, Debug, Default)]
pub struct ReporterFilter {
    pub network_id: Option<String>,
    pub account: Option<String>,
    pub role: Option<ReporterRole>,
    pub status: Option<ReporterStatus>,
    pub name: Option<String>,
    pub url: Option<String>,
}

/// Available ordering values for asset
#[derive(Enum, Copy, Clone, Eq, PartialEq, Default, Debug)]
pub enum ReporterCondition {
    /// Order by network
    NetworkId,
    /// Order by reporter id
    Id,
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
    #[default]
    UpdatedAt,
}

impl From<ReporterCondition> for Column {
    fn from(condition: ReporterCondition) -> Self {
        match condition {
            ReporterCondition::NetworkId => Column::NetworkId,
            ReporterCondition::Id => Column::Id,
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
