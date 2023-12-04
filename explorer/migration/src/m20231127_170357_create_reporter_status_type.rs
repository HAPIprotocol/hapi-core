use sea_orm::{EnumIter, Iterable};
use sea_orm_migration::{prelude::*, sea_query::extension::postgres::Type};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_type(
                Type::create()
                    .as_enum(ReporterStatus::Type)
                    .values(ReporterStatus::iter().skip(1))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_type(Type::drop().name(ReporterStatus::Type).to_owned())
            .await
    }
}

#[derive(Iden, EnumIter)]
pub enum ReporterStatus {
    #[iden = "reporter_status"]
    Type,
    Inactive,
    Active,
    Unstaking,
}
