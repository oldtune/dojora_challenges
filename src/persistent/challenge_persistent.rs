use sqlx::{PgPool, Postgres, QueryBuilder};

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
    keyword: Option<String>,
) -> Result<Vec<Challenge>, sqlx::Error> {
    let mut query_builder: QueryBuilder<'_, Postgres> = QueryBuilder::new("");
    query_builder.push("select * from challenge ");
    match keyword {
        Some(string) => {
            query_builder.push("where title like '%' || ");
            query_builder.push_bind(string);
            query_builder.push("|| '%' ");
        }
        None => (),
    };

    query_builder.push(" order by created_at offset ");
    query_builder.push_bind(page_index as i32 * page_size as i32);
    query_builder.push(" fetch next ");
    query_builder.push_bind(page_size as i32);
    query_builder.push(" rows only");

    let query = query_builder.build_query_as::<Challenge>();
    let result = query.fetch_all(db_pool).await?;

    Ok(result)
}
