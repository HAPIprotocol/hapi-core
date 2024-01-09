use {
    anyhow::Result,
    hapi_core::HapiCoreNetwork,
    jsonwebtoken::{encode, EncodingKey, Header},
    migration::{Migrator, MigratorTrait},
    sea_orm::{Database, DatabaseConnection},
    secrecy::ExposeSecret,
    secrecy::SecretString,
    std::net::SocketAddr,
    tokio::net::TcpListener,
    uuid::Uuid,
};

use anyhow::Ok;

use crate::routes::jwt_auth::TokenClaims;
use crate::{configuration::Configuration, service::Mutation};

const JWT_VALIDITY_DAYS: i64 = 365;

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

    pub async fn create_indexer(&self, network: HapiCoreNetwork) -> Result<()> {
        tracing::info!("Create indexer {}", network);

        let now = chrono::Utc::now();
        let id = Uuid::new_v4();

        Mutation::create_indexer(&self.database_conn, network, id, now).await?;

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
            &EncodingKey::from_secret(self.jwt_secret.expose_secret().as_ref()),
        )?;

        tracing::info!("IndexerId: {}. Token: {}", id, token);

        Ok(())
    }
}
