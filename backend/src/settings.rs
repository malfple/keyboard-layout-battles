use config::{Config, ConfigError, File};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AppSettings {
    pub general: General,
    pub database: Database,
}

#[derive(Debug, Deserialize)]
pub struct General {
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

    pub fn new_shuttle(database_url: String) -> Self {
        AppSettings{
            general: General{
                token_secret: String::from("test-secret"),
            },
            database: Database{
                url: database_url,
            },
        }
    }
}
