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
                    .table(Address::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Address::Id)
                            .string()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Address::Address).string().not_null())
                    .col(ColumnDef::new(Address::CaseId).uuid().not_null())
                    .col(ColumnDef::new(Address::ReporterId).uuid().not_null())
                    .col(ColumnDef::new(Address::Risk).tiny_unsigned().not_null())
                    .col(
                        ColumnDef::new(Address::Category)
                            .enumeration(Category::Type, Category::iter().skip(1))
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Address::Confirmations)
                            .big_unsigned()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Address::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Address {
    Table,
    // Network + address
    Id,
    Address,
    CaseId,
    ReporterId,
    Risk,
    Category,
    Confirmations,
}
