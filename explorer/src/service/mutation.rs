use crate::entity::FromPayload;
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
}
