use reqwest::Client;
use super::data_structures::*;
use super::*;

pub struct Discogs {
    api_endpoint: String,
    user_agent: String,
    http_client: Client
}

impl Discogs {
    pub fn new(user_agent: &str) -> Self {
        Discogs {
            api_endpoint: API_URL.to_owned(),
            user_agent: user_agent.to_owned(),
            http_client: Client::new(),
        }
    }
    
    pub fn master(&mut self, id: u64) -> Option<Master> {
        return Master::new(id,
                           self.api_endpoint.clone(),
                           self.user_agent.clone(),
                           &mut self.http_client) 
    }

    pub fn artist(&mut self, id: u64) -> Option<Artist> {
        return Artist::new(id,
                           self.api_endpoint.clone(),
                           self.user_agent.clone(),
                           &mut self.http_client)
    }

}
