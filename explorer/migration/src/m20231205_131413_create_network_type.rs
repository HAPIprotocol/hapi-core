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
                    .as_enum(Network::Type)
                    .values(Network::iter().skip(1))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_type(Type::drop().name(Network::Type).to_owned())
            .await
    }
}

#[derive(Iden, EnumIter)]
pub enum Network {
    #[iden = "network"]
    Type,
    Sepolia,
    Ethereum,
    Bsc,
    Solana,
    Bitcoin,
    Near,
}
