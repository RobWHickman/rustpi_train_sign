use rustpi_train_sign::utils;
use rustpi_train_sign::models::StationConfig;
use dotenv::dotenv;

fn main() {
    dotenv().ok();

    let test_station = StationConfig {
        name: "Wood Green".to_string(),
        id: "490001222E".to_string(),
        short_name: "Wood Green".to_string(),
        mode: "tube".to_string(),
        services: vec![],
    };

    let arrival = utils::next_arrivals(test_station).unwrap();

    println!("Next train to {} arrives at {} in {} minutes", arrival.destination, arrival.arrival, arrival.arrival_mins);
}