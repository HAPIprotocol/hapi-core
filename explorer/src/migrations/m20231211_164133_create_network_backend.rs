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
                    .as_enum(NetworkBackend::Type)
                    .values(NetworkBackend::iter().skip(1))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_type(Type::drop().name(NetworkBackend::Type).to_owned())
            .await
    }
}

#[derive(Iden, EnumIter)]
pub enum NetworkBackend {
    #[iden = "network_backend"]
    Type,
    Sepolia,
    Ethereum,
    Bsc,
    Solana,
    Bitcoin,
    Near,
}
