pub mod address;
pub mod asset;
pub mod case;
pub mod indexer;
pub mod network;
pub mod pagination;
pub mod reporter;
pub mod statistics;
pub mod types;

use self::pagination::{order_by_column, Ordering};
use sea_orm::{prelude::DateTime, EntityTrait, Select};

pub trait FromPayload<T>: Sized {
    fn from(
        network_id: String,
        created_at: Option<DateTime>,
        updated_at: Option<DateTime>,
        value: T,
    ) -> Self;
}

// Trait for Filtering query
pub trait EntityFilter: Sized + EntityTrait {
    type Filter;
    type Condition;

    fn filter(selected: Select<Self>, filter_options: &Self::Filter) -> Select<Self>;

    fn columns_for_search() -> Vec<String>;

    fn order(
        selected: Select<Self>,
        ordering: Option<Ordering>,
        condition: Option<Self::Condition>,
    ) -> Select<Self>
    where
        Self::Column: From<Self::Condition>,
        Self::Condition: Default,
    {
        order_by_column(selected, ordering, condition)
    }
}
