use {
    anyhow::{anyhow, Result},
    clap::ArgMatches,
    migration::{Migrator, MigratorTrait},
    sea_orm::{Database, DatabaseConnection},
    std::net::SocketAddr,
    std::sync::Arc,
    tokio::net::TcpListener,
};

use crate::{configuration::Configuration, entity::types::NetworkBackend, service::EntityMutation};

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
