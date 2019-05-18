use crate::discogs::Discogs;
use serde_json;
use serde::{Deserialize};
use reqwest::{Client, Error, Response};

#[derive(Deserialize, Debug)]
pub struct Search {
    pagination: Pagination,
    results: Vec<Result>,
}

impl Search {
    pub fn new( query: String, discogs: &mut Discogs) -> Option<Search> {
        let request_url: String = format!("{}/database/search?q={}", &discogs.api_endpoint, query);
        println!("{:#?}", &request_url);
        let result = discogs.query_api(&request_url);
        let search: std::result::Result<Search, Error> = result.ok()?.json();

        return None;
    }
}


#[derive(Deserialize, Debug)]
struct Result {
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
