use config::{Config, ConfigError, File};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AppSettings {
    pub general: General,
    pub database: Database,
}

#[derive(Debug, Deserialize)]
pub struct General {
    pub app_addr: String,
    pub token_secret: String,
}

#[derive(Debug, Deserialize)]
pub struct Database {
    pub url: String,
}

impl AppSettings {
    pub fn new() -> Result<Self, ConfigError> {
        let config = Config::builder()
            .add_source(File::with_name("settings/Settings.toml"))
            .build()?;

        config.try_deserialize()
    }
}
