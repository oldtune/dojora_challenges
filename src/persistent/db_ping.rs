use sqlx::PgPool;

pub async fn ping_db(db_pool: &PgPool) -> Result<(), String> {
    let result = sqlx::query(r#"select 1"#).execute(db_pool).await;
    if result.is_ok() {
        return Ok(());
    }

    return Err("Db Failed".to_string());
}
