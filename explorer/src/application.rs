use {
    anyhow::Result,
    migration::{Migrator, MigratorTrait},
    sea_orm::{Database, DatabaseConnection},
    std::net::SocketAddr,
    tokio::net::TcpListener,
    secrecy::SecretString,
};

use crate::configuration::Configuration;

pub struct Application {
    pub socket: SocketAddr,
    pub enable_metrics: bool,
    pub database_conn: DatabaseConnection,
    pub jwt_secret: SecretString,
}

impl Application {
    pub async fn from_configuration(configuration: Configuration) -> Result<Self> {
        let socket = TcpListener::bind(configuration.listener)
            .await?
            .local_addr()?;

        let database_conn = Database::connect(configuration.database_url.as_str()).await?;
        Migrator::up(&database_conn, None).await?;

        Ok(Self {
            socket,
            enable_metrics: configuration.enable_metrics,
            database_conn,
            jwt_secret: configuration.jwt_secret,
        })
    }

    pub fn port(&self) -> u16 {
        self.socket.port()
    }
}
