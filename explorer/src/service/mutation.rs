use chrono::{DateTime, NaiveDateTime, Utc};
use uuid::Uuid;

use crate::entity::{indexer, FromPayload};
use {hapi_core::HapiCoreNetwork, sea_orm::*};

pub struct Mutation;

impl Mutation {
    pub async fn create_entity<M, T>(
        db: &DbConn,
        payload: T,
        network: &HapiCoreNetwork,
    ) -> Result<<M::Entity as EntityTrait>::Model, DbErr>
    where
        <M::Entity as EntityTrait>::Model: IntoActiveModel<M>,
        M: ActiveModelBehavior + FromPayload<T> + std::marker::Send,
    {
        M::from(network, payload).insert(db).await
    }

    pub async fn update_entity<M, T>(
        db: &DbConn,
        payload: T,
        network: &HapiCoreNetwork,
    ) -> Result<<M::Entity as EntityTrait>::Model, DbErr>
    where
        <M::Entity as EntityTrait>::Model: IntoActiveModel<M>,
        M: ActiveModelBehavior + FromPayload<T> + std::marker::Send,
    {
        M::from(network, payload).update(db).await
    }

    pub async fn create_indexer(
        db: &DbConn,
        network: HapiCoreNetwork,
        id: Uuid,
        timestamp: DateTime<Utc>,
    ) -> Result<indexer::Model, DbErr> {
        indexer::ActiveModel {
            id: Set(id),
            network: Set(network.into()),
            created_at: Set(timestamp.naive_utc()),
            last_heartbeat: Set(NaiveDateTime::default()),
            cursor: Set("".to_string()),
        }
        .insert(db)
        .await
    }
}
