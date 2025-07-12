use serde::Deserialize;
use std::collections::HashMap;
use sea_orm::DbConn;

#[derive(Debug, Deserialize,Clone)]
pub struct Config {
    pub version: String,
    pub host: String,
    pub port: u16,
    pub databases: Vec<DatabaseConfig>,
}

#[derive(Debug, Deserialize,Clone)]
pub struct DatabaseConfig {
    pub key: String,
    pub database_type: String,
    pub host: String,
    pub port: u16,
    pub username: String,    
    pub password: String,
    pub database: String,
}


#[derive(Clone)]
pub struct AppState {
    pub db_map: HashMap<String, DbConn>,
}