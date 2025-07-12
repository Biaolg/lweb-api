use serde::Deserialize;
#[derive(Debug, Deserialize)]
pub struct Config {
    pub version: String,
    pub host: String,
    pub port: u16,
    pub databases: Vec<DatabaseConfig>,
}

#[derive(Debug, Deserialize)]
pub struct DatabaseConfig {
    pub key: String,
    pub database_type: String,
    pub host: String,
    pub port: u16,
    pub username: String,    
    pub password: String,
    pub database: String,
}
