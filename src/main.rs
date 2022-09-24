use std::env;

use actix_cors::Cors;
use actix_web::{middleware::Logger, web::Data, App, HttpServer};
use controllers::challenge_controller::{create_challenge, get_challenge_detail, get_challenges};
use diesel::{
    r2d2::{ConnectionManager, Pool},
    Connection, PgConnection,
};
use dotenv::dotenv;
use helpers::db_pool::DbPool;

#[macro_use]
extern crate diesel;

mod controllers;
mod helpers;
mod model;
mod response_models;
mod schema;
mod view_models;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    let connection_pool = init_connection_pool(30);
    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_header()
            .allow_any_method()
            .allow_any_origin();
        let app = App::new()
            .app_data(Data::new(connection_pool.clone()))
            .wrap(Logger::new("%a %{User-Agent}i"))
            .wrap(cors)
            .service(create_challenge)
            .service(get_challenges)
            .service(get_challenge_detail);
        app
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}

fn init_connection_pool(pool_size: u32) -> DbPool {
    let conn_string = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let conn_manager = ConnectionManager::<PgConnection>::new(&conn_string);
    let pool = Pool::builder()
        .test_on_check_out(true)
        .max_size(pool_size)
        .build(conn_manager)
        .unwrap();
    pool
}
