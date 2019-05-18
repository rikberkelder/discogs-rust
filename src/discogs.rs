use reqwest::{Client, Error, Response, RequestBuilder};
use reqwest::header::{USER_AGENT, AUTHORIZATION};
use super::data_structures::*;
use super::*;

pub struct Discogs {
    pub api_endpoint: String,
    pub user_agent: String,
    pub http_client: Client,
    pub auth_token: Option<String>,
}

impl Discogs {

    pub fn query_api (&mut self, url: &String) -> Result<Response, Error> {
        let mut api_call: RequestBuilder = self.http_client.get(url)
            .header(USER_AGENT, self.user_agent.as_str());

        if let Some(token) = self.auth_token.clone() {
            api_call = api_call.header(AUTHORIZATION, format!("Discogs token={}", token));
        }

        let result: Result<Response, Error> = api_call.send();
        return result;
    }
    
    pub fn new(user_agent: &str) -> Self {
        Discogs {
            api_endpoint: API_URL.to_owned(),
            user_agent: user_agent.to_owned(),
            http_client: Client::new(),
            auth_token: None,
        }
    }

    pub fn search(&mut self, query: String) -> Option<Search> {
        return Search::new(query, self);
    }
    
    pub fn master(&mut self, id: u64) -> Option<Master> {
        return Master::new(id, self);
    }

    pub fn artist(&mut self, id: u64) -> Option<Artist> {
        return Artist::new(id, self);   
    }
}

