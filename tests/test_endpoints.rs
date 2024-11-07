use dotenv::dotenv;
use std;
use reqwest;
use rustpi_train_sign::{
    models::{BASE, StationConfig, load_stations},
    utils::{generate_url, UrlParams},
};


#[test]
fn single_endpoint() {
    dotenv().ok();

    let tfl_api_key = std::env::var("TFL_API_KEY").expect("TFL_API_KEY not found");

    let test_station = StationConfig {
        name: "Wood Green".to_string(),
        id: "490001222E".to_string(),
        short_name: "Wood Green".to_string(),
        mode: "tube".to_string(),
        services: vec![],
    };

    let service = format!("/StopPoint/{}/Arrivals", test_station.id);

    let params = UrlParams {
        endpoint: &service,
        base_url: &BASE.base_url,
        api_key: &tfl_api_key,
    };

    let request_url: String = generate_url(params).unwrap();

    let response = reqwest::blocking::get(&request_url)
        .expect("Failed to get response");

    assert_eq!(response.status(), 200, "API request failed: {}", response.status());
}

#[test]
fn config_endpoints() {
    dotenv().ok();

    let tfl_api_key = std::env::var("TFL_API_KEY").expect("TFL_API_KEY not found");

    let mut all_passed = true;
    let stations = load_stations().expect("Failed to load stations");

    for (_, station) in stations {
        let service = format!("/StopPoint/{}/Arrivals", station.id);

        let params = UrlParams {
            endpoint: &service,
            base_url: &BASE.base_url,
            api_key: &tfl_api_key,
        };

    
        let request_url: String = generate_url(params).unwrap();

        match reqwest::blocking::get(&request_url) {
            Ok(response) => {
                if response.status() != 200 {
                    println!("Station {} failed to retrieve data with response {}", station.name, response.status());
                    all_passed = false;
                }
            },
            Err(e) => {
                println!("Station {} failed to retrieve anything! Error: {}", station.name, e);
                all_passed = false;
            }
        }

    }
    
    assert!(all_passed, "Some failed");
}