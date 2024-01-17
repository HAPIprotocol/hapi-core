use {
    anyhow::{bail, Result},
    jsonwebtoken::{encode, EncodingKey, Header},
    sea_orm::{Database, DatabaseConnection},
    sea_orm_cli::MigrateSubcommands,
    sea_orm_migration::MigratorTrait,
    secrecy::{ExposeSecret, SecretString},
    std::net::SocketAddr,
    tokio::{net::TcpListener, sync::oneshot, task::JoinHandle},
    tracing::info,
    tracing::instrument,
    uuid::Uuid,
};

use crate::{
    configuration::Configuration, entity::types::NetworkBackend, migrations::Migrator,
    server::handlers::TokenClaims, service::EntityMutation,
};

const JWT_VALIDITY_DAYS: i64 = 365;

#[derive(Clone)]
pub struct AppState {
    pub database_conn: DatabaseConnection,
    pub jwt_secret: SecretString,
}

pub struct Application {
    pub socket: SocketAddr,
    pub enable_metrics: bool,
    pub state: AppState,
    pub shutdown_sender: Option<oneshot::Sender<()>>,
    pub server_handle: Option<JoinHandle<Result<()>>>,
}

impl Application {
    pub async fn from_configuration(configuration: Configuration) -> Result<Self> {
        let socket = TcpListener::bind(configuration.listener)
            .await?
            .local_addr()?;

        let database_conn = Database::connect(configuration.database_url.as_str()).await?;

        let state = AppState {
            database_conn,
            jwt_secret: configuration.jwt_secret.to_owned(),
        };

        info!("Application initialized");

        Ok(Self {
            socket,
            enable_metrics: configuration.enable_metrics,
            state,
            shutdown_sender: None,
            server_handle: None,
        })
    }

    pub fn port(&self) -> u16 {
        self.socket.port()
    }

    #[instrument(level = "info", skip(self))]
    pub async fn migrate(&self, command: Option<MigrateSubcommands>) -> Result<()> {
        let db = &self.state.database_conn;

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
            &self.state.database_conn,
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
        EntityMutation::update_network(&self.state.database_conn, id, name, authority, stake_token)
            .await?;

        Ok(())
    }

    #[instrument(level = "info", skip(self))]
    pub async fn create_indexer(&self, network: NetworkBackend) -> Result<String> {
        tracing::info!("Create indexer for {:?} backend", network);

        let now = chrono::Utc::now();
        let id = Uuid::new_v4();

        EntityMutation::create_indexer(&self.state.database_conn, network, id, now).await?;

        let iat = now.timestamp() as usize;
        let exp = (now + chrono::Duration::days(JWT_VALIDITY_DAYS)).timestamp() as usize;
        let claims: TokenClaims = TokenClaims {
            id: id.to_string(),
            exp,
            iat,
        };

        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.state.jwt_secret.expose_secret().as_ref()),
        )?;

        tracing::info!("IndexerId: {}. Token: {}", id, token);

        Ok(token)
    }

    pub async fn shutdown(&mut self) -> Result<()> {
        // Close database connection
        self.state.database_conn.clone().close().await?;

        // Send shutdown signal
        if let Some(sender) = self.shutdown_sender.take() {
            let _ = sender.send(());
        }

        // Wait for the server task to complete
        if let Some(handle) = self.server_handle.take() {
            handle.await??;
        }

        info!("Application shutdown");
        Ok(())
    }
}
