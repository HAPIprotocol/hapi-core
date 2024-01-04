use crate::{NetworkName, ReporterRole, ReporterStatus};
use {sea_orm::Iterable, sea_orm_migration::prelude::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Reporter::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Reporter::Network)
                            .enumeration(NetworkName::Type, NetworkName::iter().skip(1))
                            .not_null(),
                    )
                    .col(ColumnDef::new(Reporter::ReporterId).uuid().not_null())
                    .col(ColumnDef::new(Reporter::Account).string().not_null())
                    .col(ColumnDef::new(Reporter::Name).string().not_null())
                    .col(ColumnDef::new(Reporter::Url).string().not_null())
                    .col(ColumnDef::new(Reporter::Stake).string().not_null())
                    .col(
                        ColumnDef::new(Reporter::Role)
                            .enumeration(ReporterRole::Type, ReporterRole::iter().skip(1))
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Reporter::Status)
                            .enumeration(ReporterStatus::Type, ReporterStatus::iter().skip(1))
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Reporter::UnlockTimestamp)
                            .string()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Reporter::CreatedAt).timestamp().not_null())
                    .col(ColumnDef::new(Reporter::UpdatedAt).timestamp().not_null())
                    .primary_key(
                        Index::create()
                            .name("reporter_id")
                            .col(Reporter::Network)
                            .col(Reporter::ReporterId),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Reporter::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub(crate) enum Reporter {
    // Composite key: network + reporter_id
    Table,
    Network,
    ReporterId,
    Account,
    Role,
    Status,
    Name,
    Url,
    Stake,
    UnlockTimestamp,
    CreatedAt,
    UpdatedAt,
}
