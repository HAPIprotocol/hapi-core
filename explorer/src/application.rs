use {
    anyhow::{bail, Result},
    sea_orm::{Database, DatabaseConnection},
    sea_orm_cli::MigrateSubcommands,
    sea_orm_migration::MigratorTrait,
    std::{net::SocketAddr, sync::Arc},
    tokio::net::TcpListener,
    tracing::instrument,
};

use crate::{
    configuration::Configuration, entity::types::NetworkBackend, migrations::Migrator,
    service::EntityMutation,
};

pub struct Application {
    pub socket: SocketAddr,
    pub enable_metrics: bool,
    pub database_conn: Arc<DatabaseConnection>,
}

impl Application {
    pub async fn from_configuration(configuration: Configuration) -> Result<Self> {
        let socket = TcpListener::bind(configuration.listener)
            .await?
            .local_addr()?;

        let database_conn = Arc::new(Database::connect(configuration.database_url.as_str()).await?);
        Migrator::up(&*database_conn, None).await?;

        Ok(Self {
            socket,
            enable_metrics: configuration.enable_metrics,
            database_conn,
        })
    }

    pub fn port(&self) -> u16 {
        self.socket.port()
    }

    #[instrument(level = "info", skip(self))]
    pub async fn migrate(&self, command: Option<MigrateSubcommands>) -> Result<()> {
        let db = &*self.database_conn;

        match command {
            None => Migrator::up(db, None).await?,
            Some(MigrateSubcommands::Up { num }) => Migrator::up(db, num).await?,
            Some(MigrateSubcommands::Fresh) => Migrator::fresh(db).await?,
            Some(MigrateSubcommands::Refresh) => Migrator::refresh(db).await?,
            Some(MigrateSubcommands::Reset) => Migrator::reset(db).await?,
            Some(MigrateSubcommands::Status) => Migrator::status(db).await?,
            Some(MigrateSubcommands::Down { num }) => Migrator::down(db, Some(num)).await?,
            _ => bail!("This command is not supported"),
        };

        Ok(())
    }

    #[instrument(level = "info", skip(self))]
    pub async fn create_network(
        &self,
        id: String,
        name: String,
        backend: NetworkBackend,
        chain_id: Option<String>,
        authority: String,
        stake_token: String,
    ) -> Result<()> {
        EntityMutation::create_network(
            &self.database_conn,
            id,
            name,
            backend,
            chain_id,
            authority,
            stake_token,
        )
        .await?;

        Ok(())
    }

    #[instrument(level = "info", skip(self))]
    pub async fn update_network(
        &self,
        id: String,
        name: Option<String>,
        authority: Option<String>,
        stake_token: Option<String>,
    ) -> Result<()> {
        EntityMutation::update_network(&self.database_conn, id, name, authority, stake_token)
            .await?;

        Ok(())
    }
}
