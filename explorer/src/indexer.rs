use crate::application::Application;
use crate::entity::indexer;
use crate::routes::jwt_auth::TokenClaims;
use anyhow::Result;
use chrono::NaiveDateTime;
use hapi_core::HapiCoreNetwork;
use jsonwebtoken::{encode, EncodingKey, Header};
use sea_orm::{ActiveModelTrait, Set};
use secrecy::ExposeSecret;
use uuid::Uuid;

const JWT_VALIDITY_DAYS: i64 = 365;

impl Application {
    pub async fn create_indexer(&self, network: HapiCoreNetwork) -> Result<()> {
        tracing::info!("Create indexer {}", network);

        let db = &self.database_conn;
        let now = chrono::Utc::now();
        let id = Uuid::new_v4();

        indexer::ActiveModel {
            id: Set(id),
            network: Set(network.into()),
            created_at: Set(now.naive_utc()),
            last_heartbeat: Set(NaiveDateTime::default()),
            cursor: Set("".to_string()),
        }
        .insert(db)
        .await?;

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
        )
        .unwrap();

        tracing::info!("IndexerId: {}. Token: {}", id, token);

        Ok(())
    }
}
