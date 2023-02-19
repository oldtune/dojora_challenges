use serde::Serialize;
use sqlx::{postgres::PgRow, FromRow, PgPool, Row};

use super::persistent_audit_metadata::Author;

#[derive(Serialize)]
pub struct Journal {
    pub userid: uuid::Uuid,
    pub id: uuid::Uuid,
    pub content: String,
    pub created_at: i64,
    pub created_by: Author,
    pub updated_at: i64,
    pub updated_by: Author,
}

#[derive(Serialize)]
pub struct JournalBrief {
    pub id: uuid::Uuid,
    #[serde(rename(serialize = "date"))]
    pub created_at: i64,
}

impl Journal {
    pub fn new(
        id: uuid::Uuid,
        userid: uuid::Uuid,
        content: String,
        created_at: i64,
        created_by: &str,
        updated_at: i64,
        updated_by: &str,
    ) -> Self {
        return Self {
            userid,
            id,
            content,
            created_at,
            created_by: created_by.into(),
            updated_at,
            updated_by: updated_by.into(),
        };
    }
}

impl FromRow<'_, PgRow> for Journal {
    fn from_row(row: &PgRow) -> Result<Self, sqlx::Error> {
        let id = row.get("id");
        let content = row.get("content");
        let created_at = row.get("created_by");
        let created_by: String = row.get("created_by");
        let updated_at = row.get("updated_at");
        let updated_by: String = row.get("updated_by");
        let user_id: uuid::Uuid = row.get("userid");

        Ok(Journal {
            content: content,
            userid: user_id,
            created_at: created_at,
            created_by: Author::from(&created_by as &str),
            id: id,
            updated_at: updated_at,
            updated_by: Author::from(&updated_by as &str),
        })
    }
}

pub async fn add_new_journal(journal: &Journal, db: &PgPool) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO Journals(ID, CONTENT, CREATED_BY, CREATED_AT, UPDATED_BY, UPDATED_AT)
        VALUES ($1, $2, $3, $4, $5, $6)"#,
        journal.id,
        journal.content,
        journal.created_by.as_ref(),
        journal.created_at as i64,
        journal.updated_by.as_ref(),
        journal.updated_at as i64
    )
    .execute(db)
    .await?;

    Ok(())
}

pub async fn get_journal(id: uuid::Uuid, db: &PgPool) -> Result<Journal, sqlx::Error> {
    let journal = sqlx::query_as_unchecked!(Journal, r#"SELECT * FROM JOURNALS WHERE ID = $1"#, id)
        .fetch_one(db)
        .await?;

    Ok(journal)
}

pub async fn get_journal_briefs(
    id: uuid::Uuid,
    db: &PgPool,
) -> Result<Vec<JournalBrief>, sqlx::Error> {
    let journal_briefs = sqlx::query_as_unchecked!(
        JournalBrief,
        "SELECT Id, CREATED_AT FROM JOURNALS WHERE USERID = $1",
        id
    )
    .fetch_all(db)
    .await;

    return journal_briefs;
}

pub async fn get_journal_detail(
    id: uuid::Uuid,
    user_id: uuid::Uuid,
    db: &PgPool,
) -> Result<Journal, sqlx::Error> {
    let journal = sqlx::query_as_unchecked!(
        Journal,
        r#"SELECT * FROM JOURNALS WHERE ID = $1 AND USERID = $2"#,
        id,
        user_id
    )
    .fetch_one(db)
    .await?;

    Ok(journal)
}
