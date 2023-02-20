use actix_web::{
    web::{self, Data, Json, Path},
    HttpResponse, Responder,
};
use anyhow::Context;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::{
    extractors::auth_extractor::AuthorisedUser,
    misc::time::get_unix_timestamp,
    persistent::{self, journal_persistent::Journal},
};

use super::route_error::ApplicationError;

#[derive(Serialize, Deserialize)]
pub struct JournalCreateModel {
    content: String,
}

#[derive(Debug, Serialize)]
pub struct JournalModel {
    content: String,
}

pub async fn add_journal(
    user: AuthorisedUser,
    journal: Json<JournalCreateModel>,
    db: Data<PgPool>,
) -> Result<impl Responder, ApplicationError> {
    println!("user id is {}", user.id);
    let id = uuid::Uuid::new_v4();
    let journal = Journal::new(
        id,
        user.id,
        journal.content.clone(),
        get_unix_timestamp(chrono::Utc::now()),
        &user.username,
        get_unix_timestamp(chrono::Utc::now()),
        &user.username,
    );

    persistent::journal_persistent::add_new_journal(&journal, db.get_ref())
        .await
        .context("Failed to insert journal into db")?;

    return Ok(HttpResponse::Ok().finish());
}

pub async fn get_journal(
    user: AuthorisedUser,
    id: Path<uuid::Uuid>,
    db: Data<PgPool>,
) -> Result<impl Responder, ApplicationError> {
    let journal = persistent::journal_persistent::get_journal(id.to_owned(), db.get_ref())
        .await
        .context("Failed to get journal from db")?;

    return Ok(HttpResponse::Ok().json(journal));
}

pub async fn get_brief_journals(
    user: AuthorisedUser,
    db: Data<PgPool>,
) -> Result<impl Responder, ApplicationError> {
    let journal_briefs = persistent::journal_persistent::get_journal_briefs(user.id, db.get_ref())
        .await
        .context("Failed to get journal brief")?;

    return Ok(web::Json(journal_briefs));
}

pub async fn get_journal_detail(
    id: Path<uuid::Uuid>,
    user: AuthorisedUser,
    db: Data<PgPool>,
) -> Result<impl Responder, ApplicationError> {
    let journal =
        persistent::journal_persistent::get_journal_detail(id.into_inner(), user.id, db.get_ref())
            .await
            .context("Failed to get journal detail")?;
    return Ok(web::Json(journal));
}
