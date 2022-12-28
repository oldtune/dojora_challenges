use actix_web::{web::Json, HttpResponse, ResponseError};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Login {
    username: String,
    password: String,
}

#[derive(thiserror::Error, Debug)]
pub enum LoginError {
    #[error("")]
    None,
}

impl ResponseError for LoginError {
    fn status_code(&self) -> reqwest::StatusCode {
        reqwest::StatusCode::BAD_REQUEST
    }
}

pub async fn login(login: Json<Login>) -> Result<HttpResponse, LoginError> {
    println!("{:?}", login);
    Err(LoginError::None)
}
