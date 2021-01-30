use melantha::config::Config;

use melantha::http::HttpResponse;

#[test]
fn send_test() {
	let config = Config::read_from("./config.toml");

	let listener = http::init(&config.port).unwrap();
	for stream in listener.incoming() {
		let stream = stream.unwrap();

		let mut res = HttpResponse::new(
			&config.root_path, 
			"/index.html", 
			stream
		).unwrap();
		res.solve().unwrap();
	}
}

use melantha::http;

#[test]
fn resolve_test() {
	let config = Config::read_from("./config.toml");

	let listener = http::init(&config.port).unwrap();
	for stream in listener.incoming() {
		let stream = stream.unwrap();

		if let Some(mut res) = http::resolve_request(
			&config.root_path, 
			stream
		) {
			println!("filetype: {}", res.filetype.as_str());
			//res.solve();
		}
	}
}
