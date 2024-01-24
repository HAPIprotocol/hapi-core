use super::{Case, Category, Network, Reporter};
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
                    .col(ColumnDef::new(Address::NetworkId).string().not_null())
                    .col(ColumnDef::new(Address::Address).string().not_null())
                    .col(ColumnDef::new(Address::CaseId).uuid().not_null())
                    .col(ColumnDef::new(Address::ReporterId).uuid().not_null())
                    .col(
                        ColumnDef::new(Address::Category)
                            .enumeration(Category::Type, Category::iter().skip(1))
                            .not_null(),
                    )
                    .col(ColumnDef::new(Address::Risk).small_integer().not_null())
                    .col(ColumnDef::new(Address::Confirmations).string().not_null())
                    .col(ColumnDef::new(Address::CreatedAt).timestamp().not_null())
                    .col(ColumnDef::new(Address::UpdatedAt).timestamp().not_null())
                    .primary_key(
                        Index::create()
                            .name("address_id")
                            .col(Address::NetworkId)
                            .col(Address::Address),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-address_network_id")
                            .from(Address::Table, Address::NetworkId)
                            .to(Network::Table, Network::Id)
                            .on_delete(ForeignKeyAction::NoAction)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-address_reporter_id")
                            .from(Address::Table, (Address::NetworkId, Address::ReporterId))
                            .to(Reporter::Table, (Reporter::NetworkId, Reporter::Id))
                            .on_delete(ForeignKeyAction::NoAction)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-address_case_id")
                            .from(Address::Table, (Address::NetworkId, Address::CaseId))
                            .to(Case::Table, (Case::NetworkId, Case::Id))
                            .on_delete(ForeignKeyAction::NoAction)
                            .on_update(ForeignKeyAction::Cascade),
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
pub(crate) enum Address {
    // Composite key: network_id + address
    Table,
    NetworkId,
    Address,
    CaseId,
    ReporterId,
    Risk,
    Category,
    Confirmations,
    CreatedAt,
    UpdatedAt,
}
