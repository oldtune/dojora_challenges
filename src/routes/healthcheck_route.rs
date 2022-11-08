use actix_web::{web::Data, HttpResponse};
use anyhow::Ok;
use sqlx::PgPool;

pub async fn health_check(db_pool: Data<PgPool>) -> HttpResponse {
    let is_ok = sqlx::query(r#"select 1"#)
        .execute(db_pool.get_ref())
        .await
        .is_ok();

    if is_ok {
        return HttpResponse::Ok().finish();
    }
    HttpResponse::BadRequest().finish()
}
