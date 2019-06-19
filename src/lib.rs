use reqwest;

pub fn fetch(url: &String) -> Result<reqwest::Response, reqwest::Error> {
	return reqwest::get(url);
}
