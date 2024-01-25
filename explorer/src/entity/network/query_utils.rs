use async_graphql::{Enum, InputObject};

use super::model::Column;
use crate::entity::types::NetworkBackend;

/// Conditions to filter address listings by
#[derive(Clone, Eq, PartialEq, InputObject, Debug)]
pub struct NetworkFilter {
    pub name: Option<String>,
    pub backend: Option<NetworkBackend>,
    pub authority: Option<String>,
    pub stake_token: Option<String>,
}

/// Available ordering values for asset
#[derive(Enum, Copy, Clone, Eq, PartialEq, Default, Debug)]
pub enum NetworkCondition {
    /// Order by id
    Id,
    /// Order by name
    Name,
    /// Order by network backend
    Backend,
    /// Order by network stake token
    StakeToken,
    /// Order by network authority
    Authority,
    /// Order by the time when entity was created
    CreatedAt,
    /// Order by the time when entity was updated
    #[default]
    UpdatedAt,
}

impl From<NetworkCondition> for Column {
    fn from(condition: NetworkCondition) -> Self {
        match condition {
            NetworkCondition::Id => Column::Id,
            NetworkCondition::Name => Column::Name,
            NetworkCondition::Backend => Column::Backend,
            NetworkCondition::StakeToken => Column::StakeToken,
            NetworkCondition::Authority => Column::Authority,
            NetworkCondition::CreatedAt => Column::CreatedAt,
            NetworkCondition::UpdatedAt => Column::UpdatedAt,
        }
    }
}
