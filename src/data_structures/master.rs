use crate::data_structures::*;
use crate::util::*;
use serde_json;
use serde::{Deserialize};
use reqwest::{Client, Error, Response};

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

#[derive(Deserialize, Debug)]
pub struct Track {
	duration: String,
	position: String,
	title: String,
	extra_artists: Option<Vec<Artist>>,
}
