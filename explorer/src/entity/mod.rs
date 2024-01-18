pub mod address;
pub mod asset;
pub mod case;
pub mod indexer;
pub mod network;
pub mod pagination;
pub mod reporter;
pub mod types;

use self::{
    pagination::{order_by_colmn, Ordering},
    types::NetworkBackend,
};
use sea_orm::{prelude::DateTime, EntityTrait, Select};

pub trait FromPayload<T>: Sized {
    fn from(
        network: NetworkBackend,
        created_at: Option<DateTime>,
        updated_at: Option<DateTime>,
        value: T,
    ) -> Self;
}

// Trait for fitlering query
pub trait EntityFilter: Sized + EntityTrait {
    type Filter;
    type Condition;

    fn filter(selected: Select<Self>, filter_options: &Self::Filter) -> Select<Self>;

    fn order(selected: Select<Self>, ordering: Ordering, condition: Self::Condition) -> Select<Self>
    where
        Self::Column: From<Self::Condition>,
    {
        order_by_colmn(selected, ordering, condition.into())
    }
}
