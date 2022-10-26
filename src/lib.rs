use actix_cors::Cors;
use actix_web::{dev::Server, middleware::Logger, web, App, HttpResponse, HttpServer};
mod routes;
use config::{Config, ConfigError, File};
use configs::global::AppConfig;
use routes::challenge_route::{add_new_challenge, get_all_challenges};
use routes::suggestion_route::{self, make_suggestion};
use sqlx::{postgres::PgPoolOptions, PgPool};
mod configs;
mod domains;
mod misc;
mod persistent;
mod request;

pub fn run(db_pool: PgPool) -> std::io::Result<Server> {
    let data_db_pool = web::Data::new(db_pool);
    let server = HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_header()
            .allow_any_method()
            .allow_any_origin();
        let app = App::new()
            .wrap(Logger::new("%a %{User-Agent}"))
            .wrap(cors)
            .route(&get_route("health_check"), web::get().to(health_check))
            .route(&get_route("challenges"), web::get().to(get_all_challenges))
            .route(&get_route("challenges"), web::post().to(add_new_challenge))
            .route(&get_route("suggestions"), web::post().to(make_suggestion))
            .app_data(data_db_pool.clone());
        app
    })
    .bind(("localhost", 8080))?
    .run();

    Ok(server)
}

pub fn get_configurations(file_name: &str) -> Result<AppConfig, ConfigError> {
    let config = Config::builder()
        .add_source(File::with_name(file_name))
        .build()?;

    Ok(config.try_deserialize::<AppConfig>()?)
}

pub async fn health_check() -> actix_web::HttpResponse {
    HttpResponse::Ok().finish()
}

pub fn get_route(route: &str) -> String {
    format!("{}{}", "api/", route)
}

#[cfg(test)]
pub mod test {
    use crate::{get_configurations, get_route};

    #[test]
    pub fn get_route_shoud_return_correct_route() {
        assert_eq!(get_route("hello"), "api/hello");
    }

    #[test]
    pub fn get_configurations_works() {
        let config = get_configurations("config.toml");
        match config {
            Ok(_) => (),
            Err(err) => panic!("{}", err),
        }
    }
}

pub async fn make_db_pool(connection_string: &str) -> Result<PgPool, sqlx::Error> {
    let connection_pool = PgPoolOptions::new()
        .max_connections(8)
        .connect(connection_string)
        .await?;

    Ok(connection_pool)
}
