use reqwest::{Client, Error, Response};
use reqwest::header::{USER_AGENT};
use serde::{Deserialize};
use serde_json;

pub const API_URL: &str = "https://api.discogs.com";

fn query_api(url: &String, user_agent: &String, client: &mut Client) -> Result<Response, Error> {
	let result: Result<Response, Error> = client.get(url).header(USER_AGENT, user_agent.as_str()).send();
	return result;
}

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

}

#[derive(Deserialize, Debug, Clone)]
pub struct Member {
	active: bool,
	id: i32,
	name: String,
	resource_url: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Artist {
	namevariations: Option<Vec<String>>,
	profile: Option<String>,
	releases_url: Option<String>,
	resource_url: Option<String>,
	uri: Option<String>,
	urls: Option<Vec<String>>,
	data_quality: Option<String>,
	id: i32,
	members: Option<Vec<Member>>,
}

impl Artist {
	pub fn new(id: u64, api_endpoint: String, user_agent: String, http_client: &mut Client) -> Option<Artist> {
		let request_url: String = format!("{}/artists/{}", api_endpoint, id);
		let result = query_api(&request_url, &user_agent, http_client);
		let artist: Artist = result.ok()?.json().ok()?;

		return Some(artist);
	}
}

#[derive(Deserialize, Debug)]
pub struct Track {
	duration: String,
	position: String,
	title: String,
	extra_artists: Option<Vec<Artist>>,
}

#[derive(Deserialize, Debug)]
pub struct Master {
	styles: Vec<String>,
	genres: Vec<String>,
	title: String,
	main_release: u64,
	main_release_url: String,
	uri: String,
	#[serde(skip)]
	artists: Vec<Option<Artist>>,
	versions_url: String,
	resource_url: String,
	tracklist: Option<Vec<Track>>,
	id: i64,
	num_for_sale: u16,
	lowest_price: f32,
	data_quality: String,
}

impl Master {
	pub fn new(id: u64, api_endpoint: String, user_agent: String, http_client: &mut Client) -> Option<Master> {
		let request_url: String = format!("{}/masters/{}", api_endpoint, id);
		let result: Result<Response, Error> = query_api(&request_url, &user_agent, http_client);
		let json: serde_json::Value = serde_json::from_str(&result.ok()?.text().ok()?).ok()?;
		let artists: &serde_json::Value = &json["artists"];
	    let mut new_artists: Vec<Option<Artist>> = Vec::new();

		for artist in &artists.as_array() {
			let id: u64 = artist[0]["id"].as_u64()?;
			new_artists.push(Artist::new(id, api_endpoint.clone(), user_agent.clone(), http_client));
		}

		let mut master: Master = serde_json::from_value(json).ok()?;

		master.artists.append(&mut new_artists);


		return Some(master);
	}
}
