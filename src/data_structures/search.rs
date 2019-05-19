use crate::discogs::{Discogs, QueryError};
use serde_json;
use serde::{Deserialize};
use reqwest::{Client, Error, Response};

#[derive(Deserialize, Debug)]
pub struct Search {
    pagination: Pagination,
    results: Vec<SearchResult>,
}

impl Search {
    pub fn new( query: String, discogs: &mut Discogs) -> Result<Search, QueryError> {
        let request_url: String = format!("{}/database/search?q={}", &discogs.api_endpoint, query);
        println!("{:#?}", &request_url);
        let mut response: Response = match discogs.query_api(&request_url) {
            Ok(response) => response,
            Err(e) => return Err(QueryError::RequestError(e)),
        };

        let status = &response.status();

        if status.is_client_error() { return Err(QueryError::ClientError(status.as_u16()))}
        if status.is_server_error() { return Err(QueryError::ServerError(status.as_u16()))}

        match response.json() {
            Ok(search) => return Ok(search),
            Err(e) => return Err(QueryError::JsonParseError(e)),
        };
    }
}


#[derive(Deserialize, Debug)]
struct SearchResult {
    #[serde(rename = "type")]
    result_type: String,
    style: Option<Vec<String>>,
    thumb: Option<String>,
    title: Option<String>,
    country: Option<String>,
    format: Option<Vec<String>>,
    uri: Option<String>,
    catno: Option<String>,
    label: Option<Vec<String>>,
    year: Option<String>,
    genre: Option<Vec<String>>,
    resource_url: Option<String>,
    id: u64,
}

#[derive(Deserialize, Debug)]
struct Pagination {
    per_page: u64,
    pages: u64,
    page: u64,
    items: u64,
    urls: Urls,
}

#[derive(Deserialize, Debug)]
struct Urls {
    next: Option<String>,
    prev: Option<String>,
} 
