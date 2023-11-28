use sea_orm::Iterable;
use sea_orm_migration::prelude::*;

use crate::m20231127_162603_create_category_type::Category;

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
                    .col(ColumnDef::new(Asset::Id).string().not_null().primary_key())
                    .col(ColumnDef::new(Asset::Address).string().not_null())
                    .col(ColumnDef::new(Asset::AssetId).string().not_null())
                    .col(ColumnDef::new(Asset::CaseId).uuid().not_null())
                    .col(ColumnDef::new(Asset::ReporterId).uuid().not_null())
                    .col(ColumnDef::new(Asset::Risk).tiny_unsigned().not_null())
                    .col(
                        ColumnDef::new(Asset::Category)
                            .enumeration(Category::Type, Category::iter().skip(1))
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Asset::Confirmations)
                            .big_unsigned()
                            .not_null(),
                    )
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
    Table,
    // Network + Asset + asset_id
    Id,
    Address,
    AssetId,
    CaseId,
    ReporterId,
    Risk,
    Category,
    Confirmations,
}
