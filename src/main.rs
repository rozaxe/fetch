use std::env;
use fetch::Request;

fn main() {
	let args: Vec<String> = env::args().collect();
    let mut args: Vec<&str> = args.iter().map(String::as_ref).collect();

	if args.len() < 2 {
		println!("usage: fetch url");
		return;
    }

    args.remove(0);

    let response = Request::from(&args).unwrap().fetch();
    println!("{:?}", response);
}
