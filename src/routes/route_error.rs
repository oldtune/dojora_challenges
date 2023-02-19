use actix_web::ResponseError;
use reqwest::StatusCode;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ApplicationError {
    #[error("Internal failure {0}")]
    InternalFailure(#[from] anyhow::Error),
    #[error("{0}")]
    BadRequest(String),
    #[error("Authorization denied")]
    Authorization,
}

impl From<&str> for ApplicationError {
    fn from(value: &str) -> Self {
        Self::BadRequest(value.to_string())
    }
}

impl ResponseError for ApplicationError {
    fn status_code(&self) -> reqwest::StatusCode {
        match self {
            Self::BadRequest(_) => StatusCode::BAD_REQUEST,
            Self::InternalFailure(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::Authorization => StatusCode::UNAUTHORIZED,
        }
    }
}
