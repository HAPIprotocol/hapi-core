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

/// The address input type
#[derive(Clone, Default, Eq, PartialEq, InputObject)]
#[graphql(concrete(name = "AddressInput", params(AddressFilter, AddressCondition)))]
pub struct EntityInput<F: InputType, C: InputType> {
    pub filtering: Option<F>,

    pub ordering: Ordering,

    pub ordering_condition: C,

    pub pagination: Option<Paginator>,
}

// /// The entity input type
// #[derive(Clone, Default, Eq, PartialEq)]
// pub struct EntityInput<OrderBy: InputObjectType, Filter: InputObjectType> {
//     pub filtering: Option<Filter>,

//     pub ordering: (Ordering, OrderBy),

//     pub pagination: Option<Paginator>,
// }
