use crate::discogs::{Discogs, QueryError};
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
	pub fn new(id: u64, discogs: &mut Discogs) -> Result<Artist, QueryError> {
		let request_url: String = format!("{}/artists/{}", &discogs.api_endpoint, id);
		let mut result = match discogs.query_api(&request_url) {
            Ok(result) => result,
            Err(e) => return Err(QueryError::RequestError(e)),
        };

        match result.json() {
            Ok(artist) => return Ok(artist),
            Err(e) => return Err(QueryError::JsonParseError(e)),
        };

	}
}
