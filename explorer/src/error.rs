use {
    axum::{
        http::StatusCode,
        response::{IntoResponse, Response},
    },
    std::fmt::Display,
};

#[derive(Debug)]
pub struct AppError {
    pub code: StatusCode,
    pub description: String,
}

impl AppError {
    pub fn new(code: StatusCode, description: String) -> Self {
        AppError { code, description }
    }

    pub fn invalid_request(description: &str) -> Self {
        AppError::new(StatusCode::BAD_REQUEST, description.to_string())
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        tracing::error!(code = ?self.code, description = ?self.description);
        (self.code, self.description).into_response()
    }
}

impl<E> From<E> for AppError
where
    E: Into<anyhow::Error> + Display,
{
    fn from(err: E) -> Self {
        Self::new(StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
    }
}
