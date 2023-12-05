use crate::{Category, Network};
use {sea_orm::Iterable, sea_orm_migration::prelude::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Asset::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Asset::Address).string().not_null())
                    .col(ColumnDef::new(Asset::AssetId).string().not_null())
                    .col(
                        ColumnDef::new(Asset::Network)
                            .enumeration(Network::Type, Network::iter().skip(1))
                            .not_null(),
                    )
                    .primary_key(
                        Index::create()
                            .name("asset_id")
                            .col(Asset::Network)
                            .col(Asset::Address)
                            .col(Asset::AssetId),
                    )
                    .col(ColumnDef::new(Asset::CaseId).uuid().not_null())
                    .col(ColumnDef::new(Asset::ReporterId).uuid().not_null())
                    .col(ColumnDef::new(Asset::Risk).small_integer().not_null())
                    .col(
                        ColumnDef::new(Asset::Category)
                            .enumeration(Category::Type, Category::iter().skip(1))
                            .not_null(),
                    )
                    .col(ColumnDef::new(Asset::Confirmations).string().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Asset::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Asset {
    // Composite key: network + address + asset_id
    Table,
    Network,
    Address,
    AssetId,
    CaseId,
    ReporterId,
    Risk,
    Category,
    Confirmations,
}
