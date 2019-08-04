
#[cfg(test)]
mod tests {
	use fetch::{is_method, format_method, Request};
	use mockito::mock;

	fn format_url(host: String) -> impl Fn(&str) -> String {
		move |route| format!("{}{}", host, route)
	}
	
	#[test]
	fn fetch_hello() {
		let url = format_url(mockito::server_url());

		let _mock = mock("GET", "/hello")
		.with_status(200)
		.create();

		Request::from(&[&url("/hello")]).unwrap().fetch();

		_mock.assert();
	}

	#[test]
	fn fetch_post() {
		let url = format_url(mockito::server_url());
		let _mock = mock("POST", "/hello")
		.with_status(200)
		.create();

		Request::from(&["POST", &url("/hello")]).unwrap().fetch();

		_mock.assert();
	}

	#[test]
	fn test_is_method() {
		assert!(is_method("GET"));
		assert!(!is_method("foo.bar"));
	}

	#[test]
	fn test_format_method() {
		assert!(format_method("GET") == "GET");
		assert!(format_method("post") == "POST");
	}

	#[test]
	fn test_format_url() {
		assert!(fetch::format_url("foo.bar") == "https://foo.bar");
		assert!(fetch::format_url("http://hell.o") == "http://hell.o");
		assert!(fetch::format_url("https://hell.o") == "https://hell.o");
	}
}
