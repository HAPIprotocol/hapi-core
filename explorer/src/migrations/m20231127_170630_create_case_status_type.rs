use {
    sea_orm::{EnumIter, Iterable},
    sea_orm_migration::{prelude::*, sea_query::extension::postgres::Type},
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_type(
                Type::create()
                    .as_enum(CaseStatus::Type)
                    .values(CaseStatus::iter().skip(1))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_type(Type::drop().name(CaseStatus::Type).to_owned())
            .await
    }
}

#[derive(Iden, EnumIter)]
pub enum CaseStatus {
    #[iden = "case_status"]
    Type,
    Closed,
    Open,
}
