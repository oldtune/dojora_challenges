use std::env;

use actix_cors::Cors;
use actix_web::{App, HttpServer};
use controllers::challenge_controller::{create_challenge, get_challenges};
use dotenv::dotenv;
use middlewares::{
    response_normalization::ResponseNormalizationMiddlewareFactory,
    validator_middleware::{self, ValidatorMiddlewareFactory},
};

#[macro_use]
extern crate diesel;

mod controllers;
mod helpers;
mod middlewares;
mod model;
mod response_models;
mod schema;
mod view_models;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    HttpServer::new(|| {
        let cors = Cors::default()
            .allow_any_header()
            .allow_any_method()
            .allow_any_origin();
        let app = App::new()
            .wrap(cors)
            // .wrap(ResponseNormalizationMiddlewareFactory)
            .service(create_challenge)
            .service(get_challenges);
        app
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
