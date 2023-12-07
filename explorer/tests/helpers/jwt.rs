use jsonwebtoken::{encode, EncodingKey, Header};

pub(crate) fn create_jwt(secret: &str) -> String {
    let claims = hapi_explorer::routes::jwt_auth::TokenClaims {
        sub: "indexer".to_string(),
        iat: 1,
        exp: 10000000000,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )
    .expect("Failed to generate JWT")
}
