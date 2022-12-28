use actix_web::{web::Data, HttpResponse};
use sqlx::PgPool;

use crate::persistent::db_ping;

pub async fn health_check(db_pool: Data<PgPool>) -> HttpResponse {
    let db_check_result = db_ping::ping_db(db_pool.get_ref()).await;

    if db_check_result.is_ok() {
        return HttpResponse::Ok().finish();
    }

    HttpResponse::BadRequest().json("Db is dead")
}
