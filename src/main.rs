use actix_web::{App, HttpServer};
use controllers::challenge_controller::create_challenge;

mod controllers;
mod events;
mod helpers;
mod response_models;
mod view_models;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(create_challenge))
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}
