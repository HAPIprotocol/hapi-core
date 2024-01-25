use super::{Network, ReporterRole, ReporterStatus};
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
                    .col(ColumnDef::new(Reporter::NetworkId).string().not_null())
                    .col(ColumnDef::new(Reporter::Id).uuid().not_null())
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
                            .col(Reporter::NetworkId)
                            .col(Reporter::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-reporter_network_id")
                            .from(Reporter::Table, Reporter::NetworkId)
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
            .drop_table(Table::drop().table(Reporter::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub(crate) enum Reporter {
    // Composite key: network_id + reporter_id
    Table,
    NetworkId,
    Id,
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
