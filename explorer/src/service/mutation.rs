use crate::entity::FromPayload;
use {chrono::NaiveDateTime, sea_orm::*, uuid::Uuid};

pub struct EntityMutation;

impl EntityMutation {
    pub async fn create_entity<M, T>(
        db: &DbConn,
        payload: T,
        network_id: Uuid,
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

        M::from(network_id, created_at, created_at, payload)
            .insert(db)
            .await
    }

    pub async fn update_entity<M, T>(
        db: &DbConn,
        payload: T,
        network_id: Uuid,
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

        M::from(network_id, None, updated_at, payload)
            .update(db)
            .await
    }
}
