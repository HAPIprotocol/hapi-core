use sea_orm::{DbConn, DbErr, EntityTrait, PrimaryKeyTrait};

pub struct Query;

impl Query {
    pub async fn find_entity_by_id<M, T>(db: &DbConn, id: T) -> Result<Option<M::Model>, DbErr>
    where
        M: EntityTrait,
        T: Into<<M::PrimaryKey as PrimaryKeyTrait>::ValueType>,
    {
        M::find_by_id(id).one(db).await
    }
}
