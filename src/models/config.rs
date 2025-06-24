use serde::Deserialize;
#[derive(Debug, Deserialize)]
pub struct Config {
    pub version: String,
    pub host: String,
    pub port: u16,
}