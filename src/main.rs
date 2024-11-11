use rustpi_train_sign::utils;
use rustpi_train_sign::models::load_stations;
use dotenv::dotenv;

fn main() {
    dotenv().ok();

    let stations = load_stations().expect("Failed to load station configurations");
    let mut all_arrivals = Vec::new();

    for (_, station) in stations {
        let api_response = utils::make_request(station.clone()).unwrap();
        // We don't need to set arrival.station here anymore since we're building it in parse_arrivals
        let arrivals = utils::parse_arrivals(api_response, &station).unwrap();
        all_arrivals.extend(arrivals);
    }

    all_arrivals.sort_by_key(|a| a.time_to_station);
    
    for arrival in all_arrivals {
        let station = arrival.station.as_ref().expect("Station should be present");
        match station.mode.as_str() {
            "tube" => println!("Next train to {} arrives at {} in {} minutes", 
                arrival.destination_name, 
                station.name,
                arrival.time_to_station),
            "bus" => println!("Next bus on route {} towards {} arrives at {} in {} minutes",
                station.services[0].line,
                arrival.destination_name, 
                station.name,
                arrival.time_to_station),
            _ => println!("Unknown mode: {}", station.mode)
        }
    }
}