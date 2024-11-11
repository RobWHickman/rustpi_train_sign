use std::fmt;

#[derive(Debug)]
pub enum TransportApiError {
   NetworkError(reqwest::Error),
   NoArrivals(String),
   InvalidData(String, String),
   UrlParseError(String, url::ParseError),
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
       }
   }
}

impl From<reqwest::Error> for TransportApiError {
   fn from(error: reqwest::Error) -> Self {
       TransportApiError::NetworkError(error)
   }
}

impl From<url::ParseError> for TransportApiError {
   fn from(error: url::ParseError) -> Self {
       TransportApiError::UrlParseError("unknown url".to_string(), error)
   }
}