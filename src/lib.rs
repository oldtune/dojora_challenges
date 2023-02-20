use actix_cors::Cors;
use actix_web::{dev::Server, middleware::Logger, web, App, HttpServer};
mod routes;
use config::{Config, ConfigError, File};
use configs::global::AppConfig;
use routes::auth_route::{register_user, test_auth, user_login};
use routes::journal_route::{add_journal, get_brief_journals, get_journal, get_journal_detail};
use sqlx::{postgres::PgPoolOptions, PgPool};
mod configs;
mod domains;
mod extractors;
mod misc;
mod persistent;
mod request;
mod responses;

pub fn run(db_pool: PgPool) -> std::io::Result<Server> {
    let data_db_pool = web::Data::new(db_pool);
    let server = HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_header()
            .allow_any_method()
            .allow_any_origin();

        let app = App::new()
            .wrap(Logger::default())
            .wrap(cors)
            .route(&get_route("journals"), web::post().to(add_journal))
            .route(
                &get_route("journals/briefs"),
                web::get().to(get_brief_journals),
            )
            .route(
                &get_route("journals/{id}"),
                web::get().to(get_journal_detail),
            )
            .route(&get_route("auth/register"), web::post().to(register_user))
            .route(&get_route("auth/login"), web::post().to(user_login))
            .route(&get_route("auth/test-token"), web::get().to(test_auth))
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
