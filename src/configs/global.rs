use serde::Deserialize;

#[derive(Deserialize)]
pub struct AppConfig {
    pub database_settings: DbConfig,
    pub running_port: u16,
}

#[derive(Deserialize)]
pub struct DbConfig {
    pub host: String,
    pub port: u16,
    pub database_name: String,
    pub username: String,
    pub password: String,
}

impl DbConfig {
    pub fn as_connection_string(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username, self.password, self.host, self.port, self.database_name
        )
    }
}
