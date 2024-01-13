use super::NetworkBackend;
use {sea_orm::Iterable, sea_orm_migration::prelude::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Indexer::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Indexer::Id).uuid().not_null().primary_key())
                    .col(
                        ColumnDef::new(Indexer::Network)
                            .enumeration(NetworkBackend::Type, NetworkBackend::iter().skip(1))
                            .not_null(),
                    )
                    .col(ColumnDef::new(Indexer::CreatedAt).timestamp().not_null())
                    .col(
                        ColumnDef::new(Indexer::LastHeartbeat)
                            .timestamp()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Indexer::Cursor).string().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Indexer::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub(crate) enum Indexer {
    Table,
    Id,
    Network,
    CreatedAt,
    LastHeartbeat,
    Cursor,
}
