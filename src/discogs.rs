use std::thread::sleep;
use std::time::Duration;
use reqwest::{Client, Error, Response, RequestBuilder};
use reqwest::header::{USER_AGENT, AUTHORIZATION};
use super::data_structures::*;
use super::*;

#[derive(Debug)]
pub enum QueryError {
    AuthenticationMissingError,
    JsonParseError (reqwest::Error),
    SerdeParseError (serde_json::Error),
    RequestError (reqwest::Error),
    
}


pub struct Discogs {
    pub api_endpoint: String,
    pub user_agent: String,
    pub http_client: Client,
    pub auth_token: Option<String>,
    pub rate_limit_left: u64,
}

impl Discogs {

    pub fn query_api (&mut self, url: &String) -> Result<Response, Error> {
        let mut api_call: RequestBuilder = self.http_client.get(url)
            .header(USER_AGENT, self.user_agent.as_str());

        if let Some(token) = self.auth_token.clone() {
            api_call = api_call.header(AUTHORIZATION, format!("Discogs token={}", token));
        }

        if self.rate_limit_left < 2 {
            sleep(Duration::from_secs(1));
        }

        let result: Result<Response, Error> = api_call.send();
        println!("{:#?}", &result);
        if let Ok(response) = &result {
            if let Some(remaining) = response.headers().get("x-discogs-ratelimit-remaining")  {
                if let Ok(remaining) = remaining.to_str() {
                    self.rate_limit_left = remaining.to_string().parse::<u64>().unwrap();
                }
            }
        }

        println!("{:#?}", &self.rate_limit_left);
        return result;
    }
    
    pub fn new(user_agent: &str) -> Self {
        Discogs {
            api_endpoint: API_URL.to_owned(),
            user_agent: user_agent.to_owned(),
            http_client: Client::new(),
            auth_token: None,
            rate_limit_left: 0,
        }
    }

    pub fn search(&mut self, query: String) -> Result<Search, QueryError>{
        return Search::new(query, self);
    }
    
    pub fn master(&mut self, id: u64) -> Result<Master, QueryError> {
        return Master::new(id, self);
    }

    pub fn artist(&mut self, id: u64) -> Result<Artist, QueryError> {
        return Artist::new(id, self);   
    }
}

