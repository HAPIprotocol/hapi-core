use sea_orm::Iterable;
use sea_orm_migration::prelude::*;

use crate::{
    m20231127_165849_create_reporter_role_type::ReporterRole,
    m20231127_170357_create_reporter_status_type::ReporterStatus,
};

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
                    .col(ColumnDef::new(Reporter::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(Reporter::Account).string().not_null())
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
                    .col(ColumnDef::new(Reporter::Name).string().not_null())
                    .col(ColumnDef::new(Reporter::Url).string().not_null())
                    .col(ColumnDef::new(Reporter::Stake).string().not_null())
                    .col(
                        ColumnDef::new(Reporter::UnlockTimestamp)
                            .big_unsigned()
                            .not_null(),
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
enum Reporter {
    Table,
    Id,
    Account,
    Role,
    Status,
    Name,
    Url,
    Stake,
    UnlockTimestamp,
}
