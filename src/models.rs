use serde::Deserialize;
use lazy_static::lazy_static;
use serde_yaml;
use std::fs;
use std::collections::HashMap;
use chrono::{DateTime, Utc};

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

#[derive(Debug, Deserialize, Clone)]
pub struct StationConfig {
    pub id: String,
    pub name: String,
    pub short_name: String,
    pub mode: String,
    pub services: Vec<ServiceConfig>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ServiceConfig {
    pub line: String,
    pub platform: String,
    pub direction: Option<String>,
}

pub fn load_stations() -> Result<HashMap<String, StationConfig>, serde_yaml::Error> {
    let config_str = fs::read_to_string("config.yaml")
        .expect("Failed to read config.yaml");
    
    let yaml: serde_yaml::Value = serde_yaml::from_str(&config_str)?;
    serde_yaml::from_value(yaml["local_stations"].clone())
}

#[derive(Debug, Deserialize)]
pub struct ArrivalData {
    #[serde(rename = "expectedArrival")]
    pub expected_arrival: DateTime<Utc>,
    #[serde(rename = "timeToStation", deserialize_with = "ArrivalData::deserialize_minutes")]
    pub time_to_station: i32,
    #[serde(rename = "destinationName")]
    pub destination_name: String,
    #[serde(skip)]
    pub station: Option<StationConfig>,
}

impl ArrivalData {
    fn deserialize_minutes<'de, D>(deserializer: D) -> Result<i32, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let seconds: i32 = serde::Deserialize::deserialize(deserializer)?;
        Ok(seconds / 60)
    }

    pub fn take_next_four(arrivals: Vec<ArrivalData>) -> Vec<ArrivalData> {
        arrivals.into_iter().take(4).collect()
    }

    pub fn sort_by_time(mut arrivals: Vec<ArrivalData>) -> Vec<ArrivalData> {
        arrivals.sort_by_key(|a| a.time_to_station);
        arrivals
    }
}


impl std::fmt::Display for ArrivalData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.time_to_station {
            0 => write!(f, "due"),
            1 => write!(f, "1min"),
            n => write!(f, "{}mins", n)
        }
    }
}


