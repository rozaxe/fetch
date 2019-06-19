	
#[cfg(test)]
mod tests {
	use fetch::fetch;
	use mockito::mock;
	
    #[test]
    fn fetch_hello() {
    	let url = &mockito::server_url();
        let _mock = mock("GET", "/hello")
          .with_status(200)
          .create();

		fetch(&format!("{}/hello", url));

        _mock.assert();
    }
}
