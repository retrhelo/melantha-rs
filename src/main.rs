// Author: Artyom Liu 

use clap::{Arg, App};

mod http;		// parse/deliver the http packet 
mod config;		// parse the given config file in TOML form 
mod mthread;	// thread pool for multi-thread 

use crate::config::Config;
use crate::mthread::ThreadPool;

use std::process::exit;

use std::sync::Mutex;
use lazy_static::*;
lazy_static! {
	static ref TP: Mutex<Option<ThreadPool>> = Mutex::new(None);
}

fn main() {
	// load configurations from command-line 
	let matches = App::new("Melantha") 
		.version(format!("{}.{}", 
			env!("CARGO_PKG_VERSION_MAJOR"), 
			env!("CARGO_PKG_VERSION_MINOR")
		).as_str())
		.author(
			format!("{}", env!("CARGO_PKG_AUTHORS"), ).as_str()
		)
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

	let config = config;		// make it constant 
	println!("port: {}", config.port);
	println!("root: {}", config.root_path);

	// initialize thread pool for multi-threading 
	*TP.lock().unwrap() = Some(ThreadPool::new(4));

	// initialize signal handler for Ctrl-C, to shutdown and exit 
	ctrlc::set_handler(|| {
		println!("{} exit, thanks for your using", env!("CARGO_PKG_NAME"));
		exit(0);
	}).unwrap();

	// initialize socket 
	let listener = http::init(&config.port).unwrap();
	for stream in listener.incoming() {
		let stream = stream.unwrap();

		if let Some(mut res) = http::resolve_request(
			&config.root_path, 
			stream
		) {
			println!("filetype: {}", res.filetype.as_str());
			TP.lock().unwrap().as_mut().unwrap().execute(move || {
				match res.solve() {
					Ok(_) => {}, 
					Err(e) => eprintln!("An error happens: {}", e), 
				};
			}).unwrap();
		}
	}
}