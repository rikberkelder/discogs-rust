use reqwest::{Client, Error, Response};
use reqwest::header::{USER_AGENT};

pub fn query_api(url: &String, user_agent: &String, client: &mut Client) -> Result<Response, Error> {
	let result: Result<Response, Error> = client.get(url).header(USER_AGENT, user_agent.as_str()).send();
	return result;
}
