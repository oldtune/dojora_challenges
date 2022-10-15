use actix_cors::Cors;
use actix_web::{
    dev::Server,
    middleware::Logger,
    web::{self, Data},
    App, HttpResponse, HttpServer,
};

pub fn run() -> std::io::Result<Server> {
    let server = HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_header()
            .allow_any_method()
            .allow_any_origin();
        let app = App::new()
            // .app_data(Data::new(connection_pool.clone()))
            .wrap(Logger::new("%a %{User-Agent}"))
            .wrap(cors)
            .route("health_check", web::get().to(health_check));
        // .service(create_challenge)
        // .service(get_challenges)
        // .service(get_challenge_detail);
        app
    })
    .bind(("localhost", 8080))?
    .run();

    Ok(server)
}

pub async fn health_check() -> actix_web::HttpResponse {
    HttpResponse::Ok().finish()
}
