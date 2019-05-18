use crate::discogs::Discogs;
use serde::Deserialize;

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
	pub fn new(id: u64, discogs: &mut Discogs) -> Option<Artist> {
		let request_url: String = format!("{}/artists/{}", &discogs.api_endpoint, id);
		let result = discogs.query_api(&request_url);
		let artist: Artist = result.ok()?.json().ok()?;

		return Some(artist);
	}
}
