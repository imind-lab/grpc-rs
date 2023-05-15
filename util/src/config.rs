use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct DBConfig {
    pub host: String,
    pub port: u32,
    pub user: String,
    pub password: String,
    pub dbname: String,
}

#[derive(Deserialize, Debug)]
pub struct CacheConfig {
    pub host: String,
    pub port: u32,
    pub password: String,
}

#[derive(Deserialize, Debug)]
pub struct Config {
    pub db: DBConfig,
    pub cache: CacheConfig,
}

impl Config {
    pub fn from_env(prefix: &str) -> Result<Self, config::ConfigError> {
        let cfg = config::Config::builder()
            .add_source(config::Environment::default()
            .prefix(prefix)
            .try_parsing(true)
            .separator(".")).build()?;
        cfg.try_deserialize()
    }
}