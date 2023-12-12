use crate::{CaseStatus, Network, Reporter};
use {sea_orm::Iterable, sea_orm_migration::prelude::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Case::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Case::CaseId).uuid().not_null())
                    .col(ColumnDef::new(Case::Network).uuid().not_null())
                    .col(ColumnDef::new(Case::Name).string().not_null())
                    .col(ColumnDef::new(Case::Url).string().not_null())
                    .col(ColumnDef::new(Case::ReporterId).uuid().not_null())
                    .col(
                        ColumnDef::new(Case::Status)
                            .enumeration(CaseStatus::Type, CaseStatus::iter().skip(1))
                            .not_null(),
                    )
                    .primary_key(
                        Index::create()
                            .name("case_id")
                            .col(Case::Network)
                            .col(Case::CaseId),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-asset_reporter_id")
                            .from(Case::Table, (Case::Network, Case::ReporterId))
                            .to(Reporter::Table, (Reporter::Network, Reporter::ReporterId))
                            .on_delete(ForeignKeyAction::NoAction)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-case_network")
                            .from(Case::Table, Case::Network)
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
            .drop_table(Table::drop().table(Case::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub(crate) enum Case {
    // Composite key: network + case_id
    Table,
    Network,
    CaseId,
    Name,
    Url,
    Status,
    ReporterId,
}
