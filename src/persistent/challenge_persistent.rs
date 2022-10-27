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
    // let query = match keyword{
    //     Some(string) =>
    //     r#"SELECT * FROM challenge where title like '%$1%' order by created_at desc offset $2 fetch next $3 rows only"#,
    // }
    // let data = sqlx::query_as_unchecked!(
    //     Challenge,
    //     keyword,
    //     (page_size * page_index) as i64,
    //     page_size as i64
    // )
    // .fetch_all(db_pool)
    // .await?;

    // Ok(data)
    let mut query_builder: QueryBuilder<'_, Postgres> = QueryBuilder::new("");
    query_builder.push("select * from challenge");
    match keyword {
        Some(string) => {
            query_builder.push(" where title like '%$1%'");
            query_builder.push_bind(string);
        }
        None => (),
    }

    query_builder.push(" order by created_at offset $2 fetch next $3 rows only");
    query_builder.push_bind((page_index * page_size) as i8);
    query_builder.push_bind(page_size as i8);

    let query = query_builder.build_query_as::<Challenge>();
    let result = query.fetch_all(db_pool).await?;

    Ok(result)
}
