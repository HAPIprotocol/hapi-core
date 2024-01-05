use {
    async_graphql::{Enum, InputObject},
    uuid::Uuid,
};

use super::model::Column;
use crate::entity::types::{CaseStatus, NetworkName};

/// Conditions to filter address listings by
#[derive(Clone, Eq, PartialEq, InputObject, Debug)]
pub struct CaseFilter {
    pub network: Option<NetworkName>,
    pub name: Option<String>,
    pub url: Option<String>,
    pub status: Option<CaseStatus>,
    pub reporter_id: Option<Uuid>,
}

/// Available ordering values for asset
#[derive(Enum, Copy, Clone, Eq, PartialEq, Default, Debug)]
pub enum CaseCondition {
    #[default]
    /// Order by network
    Network,
    /// Order by case id
    CaseId,
    /// Order by name
    Name,
    /// Order by url
    Url,
    /// Order by status
    Status,
    /// Order by reporter id
    ReporterId,
    /// Order by the time when entity was created
    CreatedAt,
    /// Order by the time when entity was updated
    UpdatedAt,
}

impl From<CaseCondition> for Column {
    fn from(condition: CaseCondition) -> Self {
        match condition {
            CaseCondition::Network => Column::Network,
            CaseCondition::CaseId => Column::CaseId,
            CaseCondition::Name => Column::Name,
            CaseCondition::Url => Column::Url,
            CaseCondition::Status => Column::Status,
            CaseCondition::ReporterId => Column::ReporterId,
            CaseCondition::CreatedAt => Column::CreatedAt,
            CaseCondition::UpdatedAt => Column::UpdatedAt,
        }
    }
}
