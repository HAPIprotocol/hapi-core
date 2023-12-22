use async_graphql::{Enum, InputObject, InputType, OutputType, SimpleObject};

use super::address::query_utils::{AddressCondition, AddressFilter};

const DEFAULT_PAGE_NUM: u64 = 1;
const DEFAULT_PAGE_SIZE: u64 = 25;

/// A convenience wrapper for pagination
#[derive(Clone, Eq, PartialEq, InputObject)]
pub struct Paginator {
    pub page_num: u64,
    pub page_size: u64,
}

impl Default for Paginator {
    fn default() -> Self {
        Self {
            page_num: DEFAULT_PAGE_NUM,
            page_size: DEFAULT_PAGE_SIZE,
        }
    }
}

/// A convenience wrapper for ordering
#[derive(Enum, Copy, Clone, Eq, PartialEq, Default)]
pub enum Ordering {
    #[default]
    Asc,
    Desc,
}

/// A paginated response for an entity
#[derive(Clone, Debug, Eq, PartialEq, SimpleObject)]
pub struct EntityPage<Entity: Send + Sync + OutputType> {
    /// The page of data being returned
    pub data: Vec<Entity>,
    /// The total number of rows available
    pub total: u64,
    /// The number of pages available
    pub page_count: u64,
}

/// Reusable input type for all entities
#[derive(Clone, Default, Eq, PartialEq, InputObject)]
#[graphql(concrete(name = "AddressInput", params(AddressFilter, AddressCondition)))]
pub struct EntityInput<F: InputType, C: InputType> {
    /// Conditions to filter entities by
    pub filtering: Option<F>,

    /// Available ordering
    pub ordering: Ordering,

    /// Available ordering values for entities
    pub ordering_condition: C,

    /// Pagination options
    pub pagination: Option<Paginator>,
}
