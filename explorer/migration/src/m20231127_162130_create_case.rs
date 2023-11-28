use sea_orm::Iterable;
use sea_orm_migration::prelude::*;

use crate::m20231127_170630_create_case_status_type::CaseStatus;

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
                    .col(ColumnDef::new(Case::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(Case::Name).string().not_null())
                    .col(ColumnDef::new(Case::Url).string().not_null())
                    .col(
                        ColumnDef::new(Case::Status)
                            .enumeration(CaseStatus::Type, CaseStatus::iter().skip(1))
                            .not_null(),
                    )
                    .col(ColumnDef::new(Case::ReporterId).uuid().not_null())
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
enum Case {
    Table,
    Id,
    Name,
    Url,
    Status,
    ReporterId,
}
