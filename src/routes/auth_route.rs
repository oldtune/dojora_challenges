use actix_web::{
    web::{self, Data, Json},
    HttpResponse, Responder,
};
use anyhow::Context;
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::{
    extractors::auth_extractor::AuthorisedUser,
    persistent::user_persistent::{self, User},
};

use super::route_error::ApplicationError;

#[derive(Serialize, Deserialize)]
pub struct UserViewModel {
    pub username: String,
    pub password: String,
}

pub async fn user_login(
    user: Json<UserViewModel>,
    db: Data<PgPool>,
) -> Result<impl Responder, ApplicationError> {
    let user = user_persistent::check_user_password(&user, db.get_ref()).await;

    if let Err(err) = &user {
        match err {
            sqlx::Error::RowNotFound => {
                return Err(ApplicationError::BadRequest(
                    "Wrong username or password".to_owned(),
                ))
            }
            _ => (),
        }
    }

    let user = user.unwrap();
    if user.is_none() {
        return Err(ApplicationError::from("Wrong username or password"));
    }

    let user = user.unwrap();
    let claims = Claims {
        id: user.id,
        username: user.username.to_owned(),
        // exp: chrono::Utc::now().timestamp(),
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(b"MeowMeowMeowMeowMeowMeowMeowMeow"),
    )
    .unwrap();

    Ok(web::Json(token))
}

pub async fn register_user(
    user: Json<User>,
    db: Data<PgPool>,
) -> Result<impl Responder, ApplicationError> {
    let user = user.into_inner();
    let user_exist = user_persistent::user_exist(&user.username, db.get_ref())
        .await
        .context("Failed to check username exist")?;

    if user_exist {
        return Err(ApplicationError::from("User existed"));
    }

    user_persistent::save_user(&user, db.get_ref())
        .await
        .context("Failed to save user")?;
    Ok(HttpResponse::Ok().finish())
}

pub async fn test_auth(auth: AuthorisedUser) -> Result<impl Responder, ApplicationError> {
    Ok(HttpResponse::Ok().finish())
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Claims {
    pub id: uuid::Uuid,
    pub username: String,
    // pub exp: i64,
}
