use crate::application::Application;
use crate::routes::jwt_auth::TokenClaims;
use hapi_core::HapiCoreNetwork;
use hapi_indexer::IndexingCursor;
use jsonwebtoken::{encode, EncodingKey, Header};
use secrecy::ExposeSecret;
use uuid::Uuid;

pub struct Indexer {
    id: Uuid,
    network: HapiCoreNetwork,
    last_heartbeat: u64,
    cursor: IndexingCursor,
}

impl Indexer {
    pub fn new(network: HapiCoreNetwork) -> Self {
        Self {
            id: Uuid::new_v4(),
            network,
            last_heartbeat: 0,
            cursor: IndexingCursor::None,
        }
    }
}

impl Application {
    pub fn create_indexer(&self, network: HapiCoreNetwork) {
        println!("Create indexer {}", network);
        let indexer = Indexer::new(network);

        // TODO: Save indexer to database

        let now = chrono::Utc::now();
        let iat = now.timestamp() as usize;
        let exp = (now + chrono::Duration::days(365)).timestamp() as usize;
        let claims: TokenClaims = TokenClaims {
            id: indexer.id.to_string(),
            exp,
            iat,
        };

        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.jwt_secret.expose_secret().as_ref()),
        )
        .unwrap();

        println!("Token: {}", token);
    }
}
