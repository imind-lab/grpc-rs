use serde::Deserialize;
use util::{CacheConfig, DBConfig};

#[derive(Deserialize, Clone, Debug)]
pub struct ServiceConfig {
    pub addr: String,
}

#[derive(Deserialize, Debug)]
pub struct Config {
    pub db: DBConfig,
    pub cache: CacheConfig,
    pub svc: ServiceConfig,
}

impl Config {
    pub fn from_env(prefix: &str) -> Result<Self, config::ConfigError> {
        let cfg = config::Config::builder()
            .add_source(
                config::Environment::default()
                    .prefix(prefix)
                    .try_parsing(true)
                    .separator("."),
            )
            .build()?;
        cfg.try_deserialize()
    }
}
