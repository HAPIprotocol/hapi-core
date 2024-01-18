use {
    async_graphql::{Enum, InputObject, InputType, OutputType, SimpleObject},
    sea_orm::{EntityTrait, QueryOrder, Select},
};

use super::{
    address::{
        model::Model as Address,
        query_utils::{AddressCondition, AddressFilter},
    },
    asset::{
        model::Model as Asset,
        query_utils::{AssetCondition, AssetFilter},
    },
    case::{
        model::Model as Case,
        query_utils::{CaseCondition, CaseFilter},
    },
    network::{
        model::Model as Network,
        query_utils::{NetworkCondition, NetworkFilter},
    },
    reporter::{
        model::Model as Reporter,
        query_utils::{ReporterCondition, ReporterFilter},
    },
};

const DEFAULT_PAGE_NUM: u64 = 1;
const DEFAULT_PAGE_SIZE: u64 = 25;

/// A convenience wrapper for pagination
#[derive(Clone, Eq, PartialEq, InputObject, Debug)]
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
#[derive(Enum, Copy, Clone, Eq, PartialEq, Default, Debug)]
pub enum Ordering {
    #[default]
    Asc,
    Desc,
}

/// Method for query ordering by column
pub fn order_by_colmn<M, C>(query: Select<M>, ordering: Ordering, condition: C) -> Select<M>
where
    M: EntityTrait,
    M::Column: From<C>,
{
    let column = M::Column::from(condition);
    match ordering {
        Ordering::Asc => query.order_by_asc(column),
        Ordering::Desc => query.order_by_desc(column),
    }
}

/// A paginated response for an entity
#[derive(Clone, Debug, Eq, PartialEq, SimpleObject)]
#[graphql(concrete(name = "NetworkPage", params(Network)))]
#[graphql(concrete(name = "ReporterPage", params(Reporter)))]
#[graphql(concrete(name = "CasePage", params(Case)))]
#[graphql(concrete(name = "AddressPage", params(Address)))]
#[graphql(concrete(name = "AssetPage", params(Asset)))]
pub struct EntityPage<Entity: Send + Sync + OutputType> {
    /// The page of data being returned
    pub data: Vec<Entity>,
    /// The total number of rows available
    pub total: u64,
    /// The number of pages available
    pub page_count: u64,
}

/// Reusable input type for all entities
#[derive(Clone, Default, Eq, PartialEq, InputObject, Debug)]
#[graphql(concrete(name = "NetworkInput", params(NetworkFilter, NetworkCondition)))]
#[graphql(concrete(name = "ReporterInput", params(ReporterFilter, ReporterCondition)))]
#[graphql(concrete(name = "CaseInput", params(CaseFilter, CaseCondition)))]
#[graphql(concrete(name = "AddressInput", params(AddressFilter, AddressCondition)))]
#[graphql(concrete(name = "AssetInput", params(AssetFilter, AssetCondition)))]
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
