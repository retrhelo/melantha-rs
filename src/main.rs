// Author: Artyom Liu 

use melantha::config::Config;

use melantha::http;

fn main() {
	let config = Config::read_from("./config.toml");

	let listener = http::init(&config.port).unwrap();
	for stream in listener.incoming() {
		let stream = stream.unwrap();

		if let Some(mut res) = http::resolve_request(
			&config.root_path, 
			stream
		) {
			println!("filetype: {}", res.filetype.as_str());
			match res.solve() {
				Ok(_) => {}, 
				Err(e) => {
					println!("an error happens: {}", e);
				}
			};
		}
	}
}
