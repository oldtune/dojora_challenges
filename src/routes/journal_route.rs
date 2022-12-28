use actix_web::{
    web::{Data, Json, Path},
    HttpResponse,
};
use anyhow::Context;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::{
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
    journal: Json<JournalCreateModel>,
    db: Data<PgPool>,
) -> Result<HttpResponse, ApplicationError> {
    let id = uuid::Uuid::new_v4();
    let journal = Journal::new(
        id,
        journal.content.clone(),
        get_unix_timestamp(chrono::Utc::now()),
        "teosuke",
        get_unix_timestamp(chrono::Utc::now()),
        "narukte",
    );

    persistent::journal_persistent::add_new_journal(&journal, db.get_ref())
        .await
        .context("Failed to insert journal into db")?;

    return Ok(HttpResponse::Ok().finish());
}

pub async fn get_journal(
    id: Path<uuid::Uuid>,
    db: Data<PgPool>,
) -> Result<HttpResponse, ApplicationError> {
    let journal = persistent::journal_persistent::get_journal(id.to_owned(), db.get_ref())
        .await
        .context("Failed to get journal from db")?;

    return Ok(HttpResponse::Ok().json(journal));
}
