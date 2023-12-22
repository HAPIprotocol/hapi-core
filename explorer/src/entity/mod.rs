pub mod address;
pub mod asset;
pub mod case;
pub mod network;
pub mod pagination;
pub mod reporter;
pub mod types;

use sea_orm::{prelude::DateTime, EntityTrait, Select};

pub trait FromPayload<T>: Sized {
    fn from(
        network_id: uuid::Uuid,
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
}
