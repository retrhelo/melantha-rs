// Author: Artyom Liu 

use clap::{Arg, App};

use melantha::config::Config;
use melantha::http;

fn main() {
	// load configurations from command-line 
	let matches = App::new("Melantha") 
		.version("0.2.0")
		.author("Artyom Liu <artyomliu@qq.com>")
		.about("A simple web server")
		.arg(Arg::with_name("config")
				.short("f")
				.long("file")
				.takes_value(true)
				.help("Config file for server to run, default as \"./config.toml\""))
		.arg(Arg::with_name("port")
				.short("p")
				.long("port")
				.takes_value(true)
				.help("Assign the port for socket to listen"))
		.arg(Arg::with_name("root_path")
				.short("r")
				.long("root")
				.takes_value(true)
				.help("Root path of server"))
		.get_matches();

	let config_file = matches.value_of("config").unwrap_or("./config.toml");
	let mut config = Config::read_from(config_file);

	if let Some(port) = matches.value_of("port") {
		config.port = String::from(port);
	}
	if let Some(root_path) = matches.value_of("root_path") {
		config.root_path = String::from(root_path);
	}

	println!("port: {}", config.port);
	println!("root: {}", config.root_path);

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
