use sqlx::PgPool;

use crate::domains::challenge::Challenge;

pub async fn insert_challenge(challenge: Challenge, db_pool: &PgPool) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO CHALLENGE (ID, TITLE, DESCRIPTION, CREATED_AT)
        VALUES ($1, $2, $3, $4)"#,
        challenge.id,
        challenge.title.as_ref(),
        challenge.description.as_ref(),
        challenge.created_at as i64,
    )
    .execute(db_pool)
    .await?;

    Ok(())
}

pub async fn query_all_challenge(
    db_pool: &PgPool,
    page_index: u8,
    page_size: u8,
) -> Result<Vec<Challenge>, sqlx::Error> {
    let data = sqlx::query_as_unchecked!(
        Challenge,
        r#"SELECT * FROM challenge order by created_at desc offset $1 fetch next $2 rows only"#,
        (page_size * page_index) as i64,
        page_size as i64
    )
    .fetch_all(db_pool)
    .await?;

    Ok(data)
}
