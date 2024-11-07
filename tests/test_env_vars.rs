use std;
use dotenv::dotenv;

// expects TFL_API_KEY and DARWIN_API_KEY to be in the environment
#[test]
fn envvars_present() {
    dotenv().ok();

    match (std::env::var("TFL_API_KEY"), std::env::var("DARWIN_API_KEY")) {
        (Ok(_), Ok(_)) => assert!(true),
        (Err(e), _) => panic!("TFL_API_KEY not found: {}", e),
        (_, Err(e)) => panic!("DARWIN_API_KEY not found: {}", e),
    }
}