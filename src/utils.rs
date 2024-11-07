// takes in parameters and generates a url to a tfl endpoint
use url;
use crate::models::{StationConfig, ArrivalData, BASE};
use reqwest;
use serde_json;
use crate::error::TransportApiError;

pub struct UrlParams<'a> {
    pub endpoint: &'a str,
    pub base_url: &'a str,
    pub api_key: &'a str,
}

pub fn generate_url(params: UrlParams) -> Result<String, url::ParseError> {
    let mut url = url::Url::parse(params.base_url)?;
    url.path_segments_mut()
        .map_err(|_| url::ParseError::EmptyHost)?
        .pop_if_empty()
        .push(params.endpoint.trim_start_matches('/'));
    
    url.query_pairs_mut().append_pair("app_key", params.api_key);
    Ok(url.to_string())
}

pub fn next_arrivals(station_details: StationConfig) -> Result<ArrivalData, TransportApiError> {
    let tfl_api_key = std::env::var("TFL_API_KEY").expect("TFL_API_KEY not found");

    let service = format!("/StopPoint/{}/Arrivals", station_details.id);
    let params = UrlParams {
        endpoint: &service,
        base_url: &BASE.base_url,
        api_key: &tfl_api_key,
    };

    let request_url = generate_url(params).unwrap();
    let response = reqwest::blocking::get(&request_url)?;
    let record: serde_json::Value = response.json()?;

    let station_name: String = station_details.name.clone();
    println!("API Response: {}", serde_json::to_string_pretty(&record).unwrap());

    if let Some(first_arrival) = record.as_array().and_then(|arr| arr.last()) {
        Ok(ArrivalData {
            station: station_details,
            arrival: first_arrival["expectedArrival"]
                .as_str()
                .ok_or_else(|| TransportApiError::InvalidData(
                    station_name.clone(), 
                    "expectedArrival".to_string()
                ))?
                .to_string(),
                arrival_mins: (first_arrival["timeToStation"]
                    .as_i64()
                    .ok_or_else(|| TransportApiError::InvalidData(
                        station_name.to_string(), 
                        "timeToStation".to_string()
                    ))? as i32) / 60,
                destination: first_arrival["destinationName"]
                .as_str()
                .ok_or_else(|| TransportApiError::InvalidData(
                    station_name.clone(), 
                    "destinationName".to_string()
                ))?
                .to_string(),
        })
    } else {
        Err(TransportApiError::NoArrivals(station_details.name))
    }
}
