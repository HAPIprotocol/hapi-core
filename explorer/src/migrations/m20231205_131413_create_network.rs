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
                    .table(Network::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Network::Id)
                            .string()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Network::Name).string().not_null())
                    .col(
                        ColumnDef::new(Network::Backend)
                            .enumeration(NetworkBackend::Type, NetworkBackend::iter().skip(1))
                            .not_null(),
                    )
                    .col(ColumnDef::new(Network::ChainId).string())
                    .col(ColumnDef::new(Network::Authority).string().not_null())
                    .col(ColumnDef::new(Network::StakeToken).string().not_null())
                    .col(ColumnDef::new(Network::CreatedAt).timestamp().not_null())
                    .col(ColumnDef::new(Network::UpdatedAt).timestamp().not_null())
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
    Backend,
    ChainId,
    Authority,
    StakeToken,
    CreatedAt,
    UpdatedAt,
}
