use crate::data_structures::*;
use crate::discogs::Discogs;
use serde_json;
use serde::{Deserialize};
use reqwest::{Error, Response};

#[derive(Deserialize, Debug)]
pub struct Master {
	styles: Vec<String>,
	genres: Vec<String>,
    videos: Vec<Video>,
	title: String,
	main_release: u64,
	main_release_url: String,
	uri: String,
	#[serde(skip)]
	artists: Vec<Option<Artist>>,
	versions_url: String,
	resource_url: String,
	tracklist: Vec<Track>,
	id: i64,
	num_for_sale: u16,
	lowest_price: f32,
	data_quality: String,
}

impl Master {
	pub fn new(id: u64, discogs: &mut Discogs) -> Option<Master> {
		let request_url: String = format!("{}/masters/{}", &discogs.api_endpoint, id);
		let result: Result<Response, Error> = discogs.query_api(&request_url);
		let json: serde_json::Value = serde_json::from_str(&result.ok()?.text().ok()?).ok()?;
		let artists: &serde_json::Value = &json["artists"];
	    let mut new_artists: Vec<Option<Artist>> = Vec::new();

		for artist in &artists.as_array() {
			let id: u64 = artist[0]["id"].as_u64()?;
            new_artists.push(discogs.artist(id));
		}

		let mut master: Master = serde_json::from_value(json).ok()?;

		master.artists.append(&mut new_artists);


		return Some(master);
	}
}

#[derive(Deserialize, Debug)]
struct Track {
	duration: String,
	position: String,
	title: String,
	extra_artists: Option<Vec<Artist>>,
}

#[derive(Deserialize, Debug)]
struct Video {
    duration: u64,
    description: String,
    embed: bool,
    uri: String,
    title: String,
}

#[derive(Deserialize, Debug)]
struct Image {
    height: u64,
    width: u64,
    resource_url: String,
    #[serde(rename = "type")]
    image_type: String,
    uri: String,
    uri150: String,
}
