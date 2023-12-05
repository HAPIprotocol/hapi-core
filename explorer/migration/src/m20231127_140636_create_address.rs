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
                    .table(Address::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Address::Address).string().not_null())
                    .col(
                        ColumnDef::new(Address::Network)
                            .enumeration(Network::Type, Network::iter().skip(1))
                            .not_null(),
                    )
                    .primary_key(
                        Index::create()
                            .name("address_id")
                            .col(Address::Network)
                            .col(Address::Address),
                    )
                    .col(ColumnDef::new(Address::CaseId).uuid().not_null())
                    .col(ColumnDef::new(Address::ReporterId).uuid().not_null())
                    .col(ColumnDef::new(Address::Risk).small_integer().not_null())
                    .col(
                        ColumnDef::new(Address::Category)
                            .enumeration(Category::Type, Category::iter().skip(1))
                            .not_null(),
                    )
                    .col(ColumnDef::new(Address::Confirmations).string().not_null())
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
    // Composite key: network + address
    Table,
    Network,
    Address,
    CaseId,
    ReporterId,
    Risk,
    Category,
    Confirmations,
}
