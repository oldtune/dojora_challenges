use sqlx::PgPool;

use crate::domains::suggestion::Suggestion;

pub async fn insert_suggestion(
    db_pool: &PgPool,
    new_suggestion: Suggestion,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"INSERT INTO suggestion (id, title, description, created_at) values($1,$2,$3,$4)"#,
        new_suggestion.id,
        new_suggestion.title.as_ref(),
        new_suggestion.description.as_ref(),
        new_suggestion.created_at,
    )
    .execute(db_pool)
    .await?;

    Ok(())
}
