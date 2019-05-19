use crate::data_structures::*;
use crate::discogs::{Discogs, QueryError};
use serde::{Deserialize};
use reqwest::{Response};

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
	artists: Vec<Artist>,
	versions_url: String,
	resource_url: String,
	tracklist: Vec<Track>,
	id: i64,
	num_for_sale: u16,
	lowest_price: f32,
	data_quality: String,
}

impl Master {
	pub fn new(id: u64, discogs: &mut Discogs) -> Result<Master, QueryError> {
		let request_url: String = format!("{}/masters/{}", &discogs.api_endpoint, id);
        let mut response: Response = match discogs.query_api(&request_url) {
            Ok(response) => response,
            Err(e) => return Err(QueryError::RequestError(e)),
        };

        let status = &response.status();

        if status.is_client_error() { return Err(QueryError::ClientError(status.as_u16()))}
        if status.is_server_error() { return Err(QueryError::ServerError(status.as_u16()))}

        match response.json() {
            Ok(master) => return Ok(master),
            Err(e) => return Err(QueryError::JsonParseError(e)),
        }
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

#[derive(Deserialize, Debug)]
struct MasterArtist {
    id: u64,
    resource_url: Option<String>,
    name: String,
}
