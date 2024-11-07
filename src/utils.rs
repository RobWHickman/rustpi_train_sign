// takes in parameters and generates a url to a tfl endpoint
use url;

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
