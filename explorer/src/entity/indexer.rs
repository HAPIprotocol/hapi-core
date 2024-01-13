use super::types::NetworkBackend;
use {sea_orm::entity::prelude::*, serde::Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize)]
#[sea_orm(table_name = "indexer")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub network: NetworkBackend,
    pub created_at: DateTime,
    pub last_heartbeat: DateTime,
    pub cursor: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
