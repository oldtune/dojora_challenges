use actix_web::{App, HttpServer};
use controllers::challenge_controller::create_challenge;

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
    HttpServer::new(|| App::new().service(create_challenge))
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}
