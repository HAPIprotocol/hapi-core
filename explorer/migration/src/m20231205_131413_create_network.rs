use crate::NetworkName;
use {sea_orm::Iterable, sea_orm_migration::prelude::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Network::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Network::Id).uuid().not_null().primary_key())
                    .col(
                        ColumnDef::new(Network::Name)
                            .enumeration(NetworkName::Type, NetworkName::iter().skip(1))
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Network::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub(crate) enum Network {
    Table,
    Id,
    Name,
}
