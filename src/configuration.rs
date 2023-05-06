use config::File;
use secrecy::{ExposeSecret, Secret};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Settings {
    pub database: DbSettings,
    pub app: AppSettings
}

#[derive(Deserialize)]
pub struct DbSettings {
    pub user_name: String,
    pub password: Secret<String>,
    pub port: u16,
    pub host: String,
    pub db_name: String,
}

#[derive(Deserialize)]
pub struct AppSettings {
    pub port: u16, 
    pub host: String
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    let mut settings = config::Config::default();
    let base_path = std::env::current_dir().expect("failed to determine current directory");
    let configs_directory = base_path.join("configuration");
    
    // read default configuration file
    settings.merge(
        File::from(configs_directory.join("base")).required(true)
    )?;

    let environment: Environment = std::env::var("APP_ENVIRONTMENT")
        .unwrap_or_else(|_| "local".into())
        .try_into()
        .expect("failed to parse APP_ENVIRONTMENT");

    settings.merge(
        File::from(configs_directory.join(environment.as_str())).required(true)
    )?;


    settings.try_into()
}

pub enum Environment {
    Local,
    Production
}

impl Environment {
    pub fn as_str(&self) -> &'static str {
        match self {
            Environment::Local => "local",
            Environment::Production => "production"
        }
    }
}

impl TryFrom<String> for Environment {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "local" => Ok(Self::Local),
            "production" => Ok(Self::Production),
            other => Err(format!(
                "{} is not a supported environment. Use either 'local' or 'production'",
                other
            ))
        }
    }
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
