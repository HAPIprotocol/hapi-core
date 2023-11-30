use sea_orm::*;

pub struct Mutation;

impl Mutation {
    pub async fn create_entity<M, T>(db: &DbConn, payload: T) -> Result<M, DbErr>
    where
        <M::Entity as EntityTrait>::Model: IntoActiveModel<M>,
        M: ActiveModelBehavior + From<T> + std::marker::Send,
    {
        M::from(payload).save(db).await
    }

    pub async fn update_entity<M, T>(
        db: &DbConn,
        payload: T,
    ) -> Result<<M::Entity as EntityTrait>::Model, DbErr>
    where
        <M::Entity as EntityTrait>::Model: IntoActiveModel<M>,
        M: ActiveModelBehavior + From<T> + std::marker::Send,
    {
        M::from(payload).update(db).await
    }
}
