use actix_cors::Cors;
use actix_web::{dev::Server, middleware::Logger, web, App, HttpResponse, HttpServer};
mod routes;
use config::{Config, ConfigError, File};
use configs::global::AppConfig;
use routes::challenge::get_all_challenges;
mod configs;
mod domains;

pub fn run() -> std::io::Result<Server> {
    let server = HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_header()
            .allow_any_method()
            .allow_any_origin();
        let app = App::new()
            .wrap(Logger::new("%a %{User-Agent}"))
            .wrap(cors)
            .route(&get_route("health_check"), web::get().to(health_check))
            .route(&get_route("challenges"), web::get().to(get_all_challenges));
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
        let config = get_configurations("src/config.toml");
        match config {
            Ok(_) => (),
            Err(err) => panic!("{}", err),
        }
    }
}
