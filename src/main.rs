use actix_web::{App, HttpServer};
use controllers::index::{get_challenges_detail, hello};

mod controllers;
mod models;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(hello).service(get_challenges_detail))
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}
