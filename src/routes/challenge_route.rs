use crate::{persistent::challenge_persistent, request::paging::PagingWithKeyword};
use actix_web::{
    web::{self, Data},
    HttpResponse, ResponseError,
};
use anyhow::Context;
use reqwest::StatusCode;
use serde::Deserialize;
use sqlx::PgPool;

use crate::{
    domains::challenge::{Challenge, ChallengeDescription, ChallengeTitle},
    misc,
};

#[derive(Debug, thiserror::Error)]
pub enum GetAllChallengeError {
    #[error("{0:?}")]
    Any(#[from] anyhow::Error),
}

impl ResponseError for GetAllChallengeError {
    fn status_code(&self) -> StatusCode {
        StatusCode::INTERNAL_SERVER_ERROR
    }
}

pub async fn get_all_challenges(
    paging: web::Query<PagingWithKeyword>,
    db_pool: Data<PgPool>,
) -> Result<HttpResponse, GetAllChallengeError> {
    let paging = paging.into_inner();

    let challenges = challenge_persistent::query_all_challenge(
        db_pool.get_ref(),
        paging.page_index,
        paging.page_size,
        paging.keyword,
    )
    .await
    .context("Failed to get challenges from db")?;

    Ok(HttpResponse::Ok().json(challenges))
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
    let description = ChallengeDescription::new(&challenge.description)?;

    let challenge = Challenge::new(
        uuid::Uuid::new_v4(),
        title,
        description,
        misc::time::get_unix_timestamp(create_date),
    );

    challenge_persistent::insert_challenge(challenge, db_pool.as_ref())
        .await
        .context("Failed to insert user into db")?;
    Ok(HttpResponse::Ok().finish())
}
