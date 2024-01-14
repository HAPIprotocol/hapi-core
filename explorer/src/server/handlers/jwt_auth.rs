use {
    axum::{
        extract::State,
        http::Request,
        http::{header, StatusCode},
        middleware::Next,
        response::IntoResponse,
        Extension,
    },
    axum_extra::extract::cookie::CookieJar,
    jsonwebtoken::{decode, DecodingKey, Validation},
    secrecy::ExposeSecret,
    serde::{Deserialize, Serialize},
};

use crate::{application::AppState, error::AppError};

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenClaims {
    pub id: String,
    pub iat: usize,
    pub exp: usize,
}

pub(crate) async fn auth_handler<B>(
    state: State<AppState>,
    cookie_jar: Extension<CookieJar>,
    req: Request<B>,
    next: Next<B>,
) -> Result<impl IntoResponse, AppError> {
    let jwt_secret = state.jwt_secret.clone();
    let token = cookie_jar
        .get("token")
        .map(|cookie| cookie.value().to_string())
        .or_else(|| {
            req.headers()
                .get(header::AUTHORIZATION)
                .and_then(|auth_header| auth_header.to_str().ok())
                .and_then(|auth_value| {
                    auth_value
                        .strip_prefix("Bearer ")
                        .map(|payload| payload.to_owned())
                })
        });

    let token = token.ok_or_else(|| {
        AppError::new(
            StatusCode::UNAUTHORIZED,
            "You are not authenticated, please provide token".to_string(),
        )
    })?;

    decode::<TokenClaims>(
        &token,
        &DecodingKey::from_secret(jwt_secret.expose_secret().as_ref()),
        &Validation::default(),
    )
    .map_err(|_| AppError::new(StatusCode::UNAUTHORIZED, "Invalid token".to_string()))?;

    Ok(next.run(req).await)
}
