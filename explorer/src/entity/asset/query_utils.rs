use {
    async_graphql::{Enum, InputObject},
    uuid::Uuid,
};

use super::model::Column;
use crate::entity::types::Category;

/// Conditions to filter address listings by
#[derive(Clone, Eq, PartialEq, InputObject, Debug, Default)]
pub struct AssetFilter {
    pub network_id: Option<String>,
    pub address: Option<String>,
    pub case_id: Option<Uuid>,
    pub reporter_id: Option<Uuid>,
    pub category: Option<Category>,
    pub risk: Option<u8>,
    pub confirmations: Option<String>,
}

/// Available ordering values for asset
#[derive(Enum, Copy, Clone, Eq, PartialEq, Default, Debug)]
pub enum AssetCondition {
    /// Order by network
    NetworkId,
    /// Order by address
    Address,
    /// Order by case id
    CaseId,
    /// Order by reporter id
    ReporterId,
    /// Order by category
    Category,
    /// Order by risk
    Risk,
    /// Order by confirmation count
    Confirmations,
    /// Order by the time when entity was created
    CreatedAt,
    /// Order by the time when entity was updated
    #[default]
    UpdatedAt,
}

impl From<AssetCondition> for Column {
    fn from(condition: AssetCondition) -> Self {
        match condition {
            AssetCondition::NetworkId => Column::NetworkId,
            AssetCondition::Address => Column::Address,
            AssetCondition::CaseId => Column::CaseId,
            AssetCondition::ReporterId => Column::ReporterId,
            AssetCondition::Category => Column::Category,
            AssetCondition::Risk => Column::Risk,
            AssetCondition::Confirmations => Column::Confirmations,
            AssetCondition::CreatedAt => Column::CreatedAt,
            AssetCondition::UpdatedAt => Column::UpdatedAt,
        }
    }
}
