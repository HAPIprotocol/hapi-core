use super::Network;
use sea_orm_migration::prelude::*;

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
                    .col(ColumnDef::new(Indexer::NetworkId).string().not_null())
                    .col(ColumnDef::new(Indexer::CreatedAt).timestamp().not_null())
                    .col(
                        ColumnDef::new(Indexer::LastHeartbeat)
                            .timestamp()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Indexer::Cursor).string().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-indexer_network_id")
                            .from(Indexer::Table, Indexer::NetworkId)
                            .to(Network::Table, Network::Id)
                            .on_delete(ForeignKeyAction::NoAction)
                            .on_update(ForeignKeyAction::Cascade),
                    )
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
    NetworkId,
    CreatedAt,
    LastHeartbeat,
    Cursor,
}
