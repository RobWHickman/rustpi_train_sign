use serde::Deserialize;
use lazy_static::lazy_static;
use serde_yaml;
use std::fs;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct Base {
    pub base_url: String,
}

impl Base {
    pub fn new() -> Self {
        let config_str = fs::read_to_string("config.yaml")
            .expect("Failed to read config.yaml");
        serde_yaml::from_str(&config_str)
            .expect("Failed to parse yaml")
    }
}

lazy_static! {
    pub static ref BASE: Base = Base::new();
}

#[derive(Debug, Deserialize)]
pub struct StationConfig {
    pub id: String,
    pub name: String,
    pub short_name: String,
    pub mode: String,
    pub services: Vec<ServiceConfig>,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum ServiceConfig {
    Bus { route: String, direction: String },
    Tube { line: String, direction: String },
    Rail { platforms: Vec<i32> },
}

pub fn load_stations() -> Result<HashMap<String, StationConfig>, serde_yaml::Error> {
    let config_str = fs::read_to_string("config.yaml")
        .expect("Failed to read config.yaml");
    
    let yaml: serde_yaml::Value = serde_yaml::from_str(&config_str)?;
    serde_yaml::from_value(yaml["local_stations"].clone())
}
