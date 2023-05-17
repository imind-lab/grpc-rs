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
