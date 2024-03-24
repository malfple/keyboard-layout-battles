use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AppSettings {
    pub app_addr: String,
    pub token_secret: String,
    pub database_url: String,
}

impl AppSettings {
    pub fn new() -> Result<Self, ConfigError> {
        let config = Config::builder()
            .add_source(File::with_name("settings/Settings.toml"))
            .add_source(Environment::default())
            .build()?;

        config.try_deserialize()
    }
}
