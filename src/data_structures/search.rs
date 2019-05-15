use crate::util::*;
use serde_json;
use serde::{Deserialize};
use reqwest::{Client, Error, Response};

#[derive(Deserialize, Debug)]
pub struct Search {
    pagination: Pagination,
    results: Vec<Result>,
}

impl Search {
    pub fn new( query: String, api_endpoint: String, user_agent: String, http_client: &mut Client) -> Option<Search> {
        let request_url: String = format!("{}/database/search?q={}", api_endpoint, query);
        println!("{:#?}", &request_url);
        let result = query_api(&request_url, &user_agent, http_client);
        let search: Search = result.ok()?.json().ok()?;

        return Some(search);
    }
}

#[derive(Deserialize, Debug)]
struct Result {
    style: Vec<String>,
    thumb: Option<String>,
    title: Option<String>,
    country: Option<String>,
    format: Vec<String>,
    uri: Option<String>,
    catno: Option<String>,
    label: Vec<String>,
    year: Option<String>,
    genre: Vec<String>,
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
