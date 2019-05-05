use reqwest::{Client, Error, Response, Url};
use serde::{Deserialize, Serialize};
use serde_json;

#[macro_use]
extern crate lazy_static;
lazy_static! {
	static ref CLIENT: Client = Client::new();
}

static API_URL: &str = "https://api.discogs.com";

fn query_api(url: &String) -> Result<Response, Error> {
	let result: Result<Response, Error> = CLIENT.get(url).send();
	return result;
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
	pub fn new(id: i64) -> Option<Artist> {
		let request_url: String = format!("{}/artists/{}", API_URL, id);
		let result = query_api(&request_url);
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
struct MasterArtist {}

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
	pub fn new(id: &i64) -> Option<Master> {
		let request_url: String = format!("{}/masters/{}", API_URL, id);
		let result: Result<Response, Error> = query_api(&request_url);
		let json: serde_json::Value = serde_json::from_str(&result.ok()?.text().ok()?).ok()?;
		let artists: &serde_json::Value = &json["artists"];
		let mut new_artists: Vec<Option<Artist>> = Vec::new();

		for artist in &artists.as_array() {
			let id: i64 = artist[0]["id"].as_i64()?;
			new_artists.push(Artist::new(id));
		}

		let mut master: Master = serde_json::from_value(json).ok()?;

		master.artists.append(&mut new_artists);

		return Some(master);
	}
}
