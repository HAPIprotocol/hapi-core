use crate::indexer::now;
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use anyhow::{anyhow, Result};

const TOKEN_DURATION: u64 = 365 * 24 * 60 * 60; // 1 year

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenClaims {
    pub sub: String,
    pub iat: usize,
    pub exp: usize,
}

pub(crate) fn create_jwt(secret: &str) -> Result<String> {

    let now = now()?;
    let claims = TokenClaims {
        sub: "indexer".to_string(),
        iat: now as usize,
        exp: (now + TOKEN_DURATION) as usize,
    };

    Ok(encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_ref()),
    ).map_err(|e| anyhow!("Failed to generate JWT: {}", e))?)
}
