use dojora::{get_configurations, make_db_pool, run};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let configuration = get_configurations("config.toml").unwrap();
    let connection_string = configuration.database_settings.as_connection_string();

    let db_pool = make_db_pool(&connection_string).await.unwrap();

    run(db_pool)?.await
}
