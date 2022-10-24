use actix_web::{
    web::{self, Data},
    HttpResponse, ResponseError,
};
use anyhow::Context;
use reqwest::StatusCode;
use serde::Deserialize;
use sqlx::PgPool;

use crate::{
    domains::challenge::{Challenge, ChallengeTitle},
    misc,
};

#[derive(Debug, thiserror::Error)]
pub enum GetAllChallengeError {
    #[error(transparent)]
    Any(#[from] anyhow::Error),
}

impl ResponseError for GetAllChallengeError {
    fn status_code(&self) -> StatusCode {
        StatusCode::INTERNAL_SERVER_ERROR
    }
}

pub async fn get_all_challenges(
    db_pool: Data<PgPool>,
) -> Result<HttpResponse, GetAllChallengeError> {
    let challenges = query_all_challenge(db_pool.get_ref())
        .await
        .context("Failed to get challenges from db")?;

    Ok(HttpResponse::Ok().json(challenges))
}

pub async fn query_all_challenge(db_pool: &PgPool) -> Result<Vec<Challenge>, sqlx::Error> {
    let data = sqlx::query_as_unchecked!(Challenge, r#"SELECT * FROM challenge"#)
        .fetch_all(db_pool)
        .await?;

    Ok(data)
}

#[derive(Deserialize)]
pub struct NewChallenge {
    title: String,
    description: String,
}

#[derive(thiserror::Error, Debug)]
pub enum CreateChallengeError {
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
    #[error("{0}")]
    Validation(String),
}

impl From<String> for CreateChallengeError {
    fn from(validation: String) -> Self {
        Self::Validation(validation)
    }
}

impl ResponseError for CreateChallengeError {
    fn status_code(&self) -> reqwest::StatusCode {
        match self {
            Self::UnexpectedError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::Validation(_) => StatusCode::BAD_REQUEST,
        }
    }
}

pub async fn add_new_challenge(
    challenge: web::Json<NewChallenge>,
    db_pool: Data<PgPool>,
) -> Result<HttpResponse, CreateChallengeError> {
    let create_date = chrono::Utc::now();
    let title = ChallengeTitle::new(&challenge.title)?;

    let challenge = Challenge::new(
        uuid::Uuid::new_v4(),
        title,
        challenge.description.to_string(),
        misc::time::get_unix_timestamp(create_date),
    );

    insert_challenge(challenge, db_pool.as_ref())
        .await
        .context("Failed to insert user into db")?;
    Ok(HttpResponse::Ok().finish())
}

async fn insert_challenge(challenge: Challenge, db_pool: &PgPool) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO CHALLENGE (ID, TITLE, DESCRIPTION, CREATED_AT)
        VALUES ($1, $2, $3, $4)"#,
        challenge.id,
        challenge.title.as_ref(),
        challenge.description,
        challenge.created_at as i64,
    )
    .execute(db_pool)
    .await?;

    Ok(())
}
