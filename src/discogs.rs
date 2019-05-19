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
    ClientError (u16),
    ServerError (u16),
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



#[cfg(test)]
mod tests {
    use crate::discogs::*;

    fn get_discogs() -> Discogs {
        return Discogs::new("discogs-rust-unit-test/1");
    }

    #[test]
    fn test_discogs_query_api() {
        let mut discogs = get_discogs();

        match discogs.query_api(&String::from("https://api.discogs.com/releases/249504")) {
            Ok(r) => assert!(true),
            Err(e) => assert!(false, "Error making request"),
        }
    }

    #[test]
    fn test_discogs_query_api_invalid_url() {
        let mut discogs = get_discogs();

        match discogs.query_api(&String::from("hello!")) {
            Ok(r) => assert!(false),
            Err(e) => assert!(true, "Error making request"),
        }
    }

    #[test]
    fn test_get_master() {
        let mut discogs = get_discogs();

        match discogs.master(666) {
            Ok(r) => assert!(true),
            Err(e) => assert!(false),
        }
    }

    #[test]
    fn test_get_nonexistent_master(){
        let mut discogs = get_discogs();
        
        match discogs.master(1) {
            Ok(r) => assert!(false),
            Err(e) => match e {
                QueryError::ClientError(e) => assert!(true),
                _ => assert!(false),
            },
        }
    }

    #[test]
    fn test_get_artist() {
        let mut discogs = get_discogs();

        match discogs.artist(666) {
            Ok(r) => assert!(true),
            Err(e) => assert!(false),
        }
    }

    #[test]
    fn test_get_nonexistent_artist(){
        let mut discogs = get_discogs();
        
        match discogs.artist(99999999999) {
            Ok(r) => assert!(false),
            Err(e) => match e {
                QueryError::ClientError(e) => assert!(true),
                _ => assert!(false),
            },
        }
    }

    #[test]
    fn test_get_search_no_auth(){
        let mut discogs = get_discogs();
        
        match discogs.search(String::from("Metallica")) {
            Ok(r) => assert!(false),
            Err(e) => match e {
                QueryError::ClientError(e) => assert!(true),
                _ => assert!(false),
            },
        }
    }

    
}
