use crate::{
    domains::suggestion::{Suggestion, SuggestionDescription},
    misc,
};
use actix_web::{
    web::{self, Data},
    HttpResponse, ResponseError,
};
use anyhow::Context;
use serde::Deserialize;
use sqlx::PgPool;

#[derive(Deserialize)]
pub struct NewSuggestion {
    title: String,
    description: String,
}

#[derive(thiserror::Error, Debug)]
pub enum InsertSuggestionError {
    #[error(transparent)]
    Unexpected(#[from] anyhow::Error),
    #[error("{0}")]
    Validation(String),
}

impl ResponseError for InsertSuggestionError {
    fn status_code(&self) -> reqwest::StatusCode {
        match self {
            Self::Unexpected(_) => reqwest::StatusCode::INTERNAL_SERVER_ERROR,
            Self::Validation(_) => reqwest::StatusCode::BAD_REQUEST,
        }
    }
}

pub async fn make_suggestion(
    db_pool: Data<PgPool>,
    new_suggestion: web::Json<NewSuggestion>,
) -> Result<HttpResponse, InsertSuggestionError> {
    let new_suggestion = new_suggestion.into_inner();
    let suggestion_desc = SuggestionDescription::new(new_suggestion.description)
        .map_err(|err| InsertSuggestionError::Validation(err))?;

    let suggestion = Suggestion::new(
        uuid::Uuid::new_v4(),
        new_suggestion.title,
        suggestion_desc,
        misc::time::get_unix_timestamp(chrono::Utc::now()),
    );

    insert_suggestion(db_pool.get_ref(), suggestion)
        .await
        .context("Failed to insert suggestion into db")?;

    Ok(HttpResponse::Ok().finish())
}

pub async fn insert_suggestion(
    db_pool: &PgPool,
    new_suggestion: Suggestion,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"INSERT INTO suggestion (id, title, description, created_at) values($1,$2,$3,$4)"#,
        new_suggestion.id,
        new_suggestion.title,
        new_suggestion.description.as_ref(),
        new_suggestion.created_at,
    )
    .execute(db_pool)
    .await?;

    Ok(())
}

pub async fn view_list_suggestion() -> Result<HttpResponse, String> {
todo!() 
}

pub async fn view_suggestion(id: uuid::Uuid) -> Result<HttpResponse, anyhow::Error> {
    todo!()
}

pub async fn vote_suggestion(id: uuid::Uuid) -> Result<HttpResponse, String> {
    //transaction here
    todo!()
}
