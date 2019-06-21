use std::env;
use fetch::Request;

fn print_usage() {
	println!("usage: fetch url");
}

fn main() {
	let args: Vec<String> = env::args().collect();
    let mut args: Vec<&str> = args.iter().map(String::as_ref).collect();

	if args.len() < 2 {
        print_usage();
		return;
    }

    args.remove(0);

    match Request::from(&args) {
        Some(request) => match request.fetch() {
            Ok(mut response) => {

                // Print headers
                for (key, value) in response.headers().iter() {
                    println!("{}: {}", key, value.to_str().unwrap());
                }

                // Print body
                match response.text() {
                    Ok(body) => println!("{}", body),
                    Err(_) => {}
                }
            },
            Err(err) => println!("{}", err),
        },
        None => print_usage()
    }
}
