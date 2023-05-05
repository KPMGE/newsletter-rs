use serde::Deserialize;
use secrecy::{Secret, ExposeSecret};

#[derive(Deserialize)]
pub struct Settings {
    pub database: DbSettings,
    pub app_port: u16,
    pub app_host: String
}

#[derive(Deserialize)]
pub struct DbSettings {
    pub user_name: String, 
    pub password: Secret<String>, 
    pub port: u16,
    pub host: String,
    pub db_name: String
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    let mut settings = config::Config::default();
    settings.merge(config::File::with_name("configuration"))?;
    settings.try_into()
}

impl DbSettings {
    pub fn get_connection_string(&self) -> Secret<String> {
        let connection_str = format!(
            "postgres://{}:{}@{}:{}/{}",
            self.user_name,
            self.password.expose_secret(), 
            self.host, 
            self.port, 
            self.db_name
        );
        Secret::new(connection_str)
    }

    pub fn get_connection_string_without_db(&self) -> Secret<String> {
        let connection_str = format!(
            "postgres://{}:{}@{}:{}",
            self.user_name, 
            self.password.expose_secret(), 
            self.host,
            self.port
        );
        Secret::new(connection_str)
    }
}
