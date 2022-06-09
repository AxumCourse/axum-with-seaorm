use serde::Deserialize;

#[derive(Deserialize)]
pub struct WebConfig {
    pub addr: String,
}
#[derive(Deserialize)]
pub struct DbConfig {
    pub dsn: String,
}

#[derive(Deserialize)]
pub struct Config {
    pub web: WebConfig,
    pub db: DbConfig,
}

impl Config {
    pub fn from_env() -> Result<Self, config::ConfigError> {
        config::Config::builder()
            .add_source(config::Environment::default())
            .build()?
            .try_deserialize()
    }
}
