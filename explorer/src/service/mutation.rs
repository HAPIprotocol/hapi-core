use crate::entity::FromPayload;
use {sea_orm::*, uuid::Uuid};

pub struct Mutation;

impl Mutation {
    pub async fn create_entity<M, T>(
        db: &DbConn,
        payload: T,
        network_id: Uuid,
    ) -> Result<<M::Entity as EntityTrait>::Model, DbErr>
    where
        <M::Entity as EntityTrait>::Model: IntoActiveModel<M>,
        M: ActiveModelBehavior + FromPayload<T> + std::marker::Send,
    {
        M::from(network_id, payload).insert(db).await
    }

    pub async fn update_entity<M, T>(
        db: &DbConn,
        payload: T,
        network_id: Uuid,
    ) -> Result<<M::Entity as EntityTrait>::Model, DbErr>
    where
        <M::Entity as EntityTrait>::Model: IntoActiveModel<M>,
        M: ActiveModelBehavior + FromPayload<T> + std::marker::Send,
    {
        M::from(network_id, payload).update(db).await
    }
}
