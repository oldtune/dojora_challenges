use crate::domains::suggesstion::{Suggestion, SuggestionDescription};
use actix_web::{web, HttpResponse};
use sqlx::PgPool;
pub struct NewSuggestion {
    title: String,
    description: String,
}

pub async fn make_suggestion(
    new_suggestion: web::Json<NewSuggestion>,
) -> Result<HttpResponse, String> {
    let new_suggestion = new_suggestion.into_inner();

    let suggestion = Suggestion::new(
        new_suggestion.title,
        SuggestionDescription::new(new_suggestion.description),
    );

    todo!()
}

pub async fn insert_suggestion(
    db_pool: &PgPool,
    new_suggestion: NewSuggestion,
) -> Result<(), sqlx::Error> {
    todo!()
}

pub async fn view_suggestion() -> Result<HttpResponse, String> {
    todo!()
}

pub async fn vote_suggestion(id: uuid::Uuid) -> Result<HttpResponse, String> {
    //transaction here
    todo!()
}
