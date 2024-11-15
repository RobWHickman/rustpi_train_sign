use std::fmt;
use std::io;

#[derive(Debug)]
pub enum TransportApiError {
    NetworkError(String),
    NoArrivals(String),
    InvalidData(String, String),
    UrlParseError(String, url::ParseError),
    JsonError(String),  // Add this variant
}

impl fmt::Display for TransportApiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TransportApiError::NetworkError(ref err) => write!(f, "Network error: {}", err),
            TransportApiError::NoArrivals(ref station) => write!(f, "No arrivals found for {}", station),
            TransportApiError::InvalidData(ref station, ref missing_data) => 
                write!(f, "Invalid {} data for {}", missing_data, station),
            TransportApiError::UrlParseError(ref url, ref err) => 
                write!(f, "Failed to parse URL '{}': {}", url, err),
            TransportApiError::JsonError(ref err) => write!(f, "JSON error: {}", err),  // Add this
        }
    }
}

impl From<ureq::Error> for TransportApiError {
    fn from(error: ureq::Error) -> Self {
        TransportApiError::NetworkError(error.to_string())
    }
}

impl From<url::ParseError> for TransportApiError {
    fn from(error: url::ParseError) -> Self {
        TransportApiError::UrlParseError("unknown url".to_string(), error)
    }
}

impl From<io::Error> for TransportApiError {
    fn from(error: io::Error) -> Self {
        TransportApiError::JsonError(error.to_string())
    }
}

impl From<serde_json::Error> for TransportApiError {
    fn from(error: serde_json::Error) -> Self {
        TransportApiError::JsonError(error.to_string())
    }
}