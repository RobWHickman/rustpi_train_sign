// takes in parameters and generates a url to a tfl endpoint
use url;
use crate::models::{ArrivalData, StationConfig, ServiceConfig};
use ureq;
use dotenv;
use serde_json;
use crate::error::TransportApiError;
use crate::models::BASE;

pub struct UrlParams<'a> {
    pub endpoint: &'a str,
    pub base_url: &'a str,
    pub api_key: &'a str,
}

pub fn make_request(station_details: StationConfig) -> Result<serde_json::Value, TransportApiError> {
    dotenv::dotenv().ok();

    let service = format!("/StopPoint/{}/Arrivals", station_details.id);

    let station_url_params = UrlParams {
        endpoint: &service,
        base_url: &BASE.base_url,
        api_key: &std::env::var("TFL_API_KEY").expect("TFL_API_KEY not set"),
    };

    let request_url = generate_url(station_url_params)?;
    let response = ureq::get(&request_url)
        .call()?;
    let record: serde_json::Value = response.into_json()?;  // Changed from json() to into_json()
    Ok(record)
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

pub fn parse_arrivals(record: serde_json::Value, station: &StationConfig) -> Result<Vec<ArrivalData>, TransportApiError> {
    let arrivals = record.as_array()
        .ok_or_else(|| TransportApiError::InvalidData(
            "root".to_string(), 
            "Expected array of predictions".to_string()
        ))?;

    if arrivals.is_empty() {
        return Err(TransportApiError::NoArrivals(
            "Empty array received from API".to_string()
        ));
    }

    let mut arrival_data: Vec<ArrivalData> = arrivals.iter()
        .filter(|arrival| {
            station.services.iter().any(|service| {
                arrival["lineId"].as_str() == Some(&service.line) && 
                arrival["platformName"].as_str() == Some(&service.platform)
            })
        })
        .filter_map(|arrival| {
            let mode = arrival["modeName"].as_str()?;
            let mut arrival_data: ArrivalData = serde_json::from_value(arrival.clone()).ok()?;
            let service_config = station.services.iter().find(|service| {
                arrival["lineId"].as_str() == Some(&service.line) && 
                arrival["platformName"].as_str() == Some(&service.platform)
            })?;

            let direction = match mode {
                "tube" => service_config.direction.clone(),
                "bus" => Some(arrival["destinationName"].as_str()?.to_string()),
                _ => None,
            };

            let station_config = StationConfig {
                id: arrival["naptanId"].as_str()?.to_string(),
                name: arrival["stationName"].as_str()?.to_string(),
                short_name: station.short_name.clone(),
                mode: mode.to_string(),
                services: vec![ServiceConfig {
                    line: arrival["lineId"].as_str()?.to_string(),
                    platform: arrival["platformName"].as_str()?.to_string(),
                    direction,  // Use our determined direction
                }],
            };
            arrival_data.station = Some(station_config);
            Some(arrival_data)
        })
        .collect();

    arrival_data = ArrivalData::take_next_four(ArrivalData::sort_by_time(arrival_data));

    Ok(arrival_data)
}