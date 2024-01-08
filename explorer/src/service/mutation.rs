use crate::entity::{types::NetworkBackend, FromPayload};
use {chrono::NaiveDateTime, sea_orm::*};

pub struct EntityMutation;

impl EntityMutation {
    /// Universal method for inserting entities to database
    pub async fn create_entity<M, T>(
        db: &DbConn,
        payload: T,
        network: NetworkBackend,
        timestamp: u64,
    ) -> Result<<M::Entity as EntityTrait>::Model, DbErr>
    where
        <M::Entity as EntityTrait>::Model: IntoActiveModel<M>,
        M: ActiveModelBehavior + FromPayload<T> + Send,
    {
        let created_at = Some(
            NaiveDateTime::from_timestamp_opt(timestamp as i64, 0)
                .ok_or(DbErr::Custom("Invalid block timestamp".to_string()))?,
        );

        M::from(network, created_at, created_at, payload)
            .insert(db)
            .await
    }

    /// Universal method for updating entities in database
    pub async fn update_entity<M, T>(
        db: &DbConn,
        payload: T,
        network: NetworkBackend,
        timestamp: u64,
    ) -> Result<<M::Entity as EntityTrait>::Model, DbErr>
    where
        <M::Entity as EntityTrait>::Model: IntoActiveModel<M>,
        M: ActiveModelBehavior + FromPayload<T> + Send,
    {
        let updated_at = Some(
            NaiveDateTime::from_timestamp_opt(timestamp as i64, 0)
                .ok_or(DbErr::Custom("Invalid block timestamp".to_string()))?,
        );

        M::from(network, None, updated_at, payload).update(db).await
    }
}
