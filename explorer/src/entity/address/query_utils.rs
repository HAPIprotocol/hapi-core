use {
    async_graphql::{Enum, InputObject},
    uuid::Uuid,
};

use super::model::Column;
use crate::entity::types::Category;

/// Conditions to filter address listings by
#[derive(Clone, Eq, PartialEq, InputObject, Debug, Default)]
pub struct AddressFilter {
    pub network_id: Option<String>,
    pub case_id: Option<Uuid>,
    pub reporter_id: Option<Uuid>,
    pub category: Option<Category>,
    pub risk: Option<u8>,
    pub confirmations: Option<String>,
}

/// Available ordering values for address
#[derive(Enum, Copy, Clone, Eq, PartialEq, Default, Debug)]
pub enum AddressCondition {
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

impl From<AddressCondition> for Column {
    fn from(condition: AddressCondition) -> Self {
        match condition {
            AddressCondition::NetworkId => Column::NetworkId,
            AddressCondition::Address => Column::Address,
            AddressCondition::CaseId => Column::CaseId,
            AddressCondition::ReporterId => Column::ReporterId,
            AddressCondition::Category => Column::Category,
            AddressCondition::Risk => Column::Risk,
            AddressCondition::Confirmations => Column::Confirmations,
            AddressCondition::CreatedAt => Column::CreatedAt,
            AddressCondition::UpdatedAt => Column::UpdatedAt,
        }
    }
}
