use serde::Deserialize;

#[derive(Deserialize)]
pub struct AppConfig {
    database_settings: DbConfig,
    running_port: u16,
}

#[derive(Deserialize)]
pub struct DbConfig {
    host: String,
    port: u16,
    database_name: String,
    username: String,
    password: String,
}
