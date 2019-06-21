
#[cfg(test)]
mod tests {
	use fetch::{is_method, Request};
	use mockito::mock;
	
    #[test]
    fn fetch_hello() {
    	let url = &mockito::server_url();
        let _mock = mock("GET", "/hello")
          .with_status(200)
          .create();

        Request::from(&[&format!("{}/hello", url)]).unwrap().fetch();

        _mock.assert();
    }

    #[test]
    fn fetch_post() {
    	let url = &mockito::server_url();
        let _mock = mock("POST", "/hello")
          .with_status(200)
          .create();

        Request::from(&["POST", &format!("{}/hello", url)]).unwrap().fetch();

        _mock.assert();
    }
    
    #[test]
    fn parse_method() {
    	assert!(is_method("GET"));
    	assert!(!is_method("foo.bar"));
    }
}
