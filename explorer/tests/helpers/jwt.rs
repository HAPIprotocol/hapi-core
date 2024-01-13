use {
    hapi_explorer::server::TokenClaims,
    jsonwebtoken::{encode, EncodingKey, Header},
};

pub(crate) fn create_jwt(secret: &str) -> String {
    let claims = TokenClaims {
        id: get_jwt_id(),
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

pub fn get_jwt_id() -> String {
    "1466cf4f-1d71-4153-b9ad-4a9c1b48101e".to_string()
}
