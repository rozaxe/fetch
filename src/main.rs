use std::env;
use fetch::fetch;

fn main() {
	let args: Vec<_> = env::args().collect();
	
	if args.len() != 2 {
		println!("usage: fetch url");
		return;
    }
    
    let response = fetch(&args[1]);
    println!("{:?}", response);
}
