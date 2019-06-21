#[macro_use] extern crate lazy_static;
use reqwest;
use regex::Regex;

#[derive(Debug)]
pub struct Error {}

#[derive(Debug)]
pub struct Response {}

pub struct Request {
	pub method: String,
	pub url: String,
}

enum Token {
	METHOD,
	URL,
}

fn parse_token(token: &str) -> Token {
	if is_method(token) {
		return Token::METHOD;
	}
	
	return Token::URL;
}

fn to_full_url(url: &str) -> String {
    lazy_static! {
        static ref HTTP_REGEX: Regex = Regex::new("^https?://").unwrap();
    }

    if HTTP_REGEX.is_match(url) {
        return String::from(url);
    }

    return format!("https://{}", url);
}

impl Request {
	pub fn from(tokens: &[&str]) -> Option<Request> {
	
		let mut method: String = String::from("GET");
		let mut url: Option<String> = None;
		
		for token in tokens {
			match parse_token(token) {
				Token::METHOD => method = token.to_uppercase(),
				Token::URL => url = Some(to_full_url(token)),
			}
		}

        return match url {
            Some(url) => Some(Request {
    		    method: method,
    		    url: url,
    	    }),
            None => None,
        };
	}

    pub fn fetch(&self) -> Result<Response, Error> {
        let method = reqwest::Method::from_bytes(self.method.as_bytes()).unwrap();
        let builder = reqwest::Client::new().request(method, &self.url);

	    let response = match builder.send() {
		    Ok(_) => Ok(Response {}),
		    Err(_) => Err(Error {}),
	    };
	    return response;
    }
}

pub fn is_method(potential: &str) -> bool {
	let methods = vec![
		"OPTIONS",
		"GET",
		"HEAD",
		"POST",
		"PUT",
		"DELETE",
		"TRACE",
		"CONNECT",
	];

	return methods.contains(&potential.to_uppercase().as_str());
}

