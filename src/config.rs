// Author: Artyom Liu 

use toml::de::from_str;

use serde_derive::Deserialize;

#[derive(Deserialize)]
pub struct Config {
	pub port: String, 
	pub root_path: String, 
}

use std::fs::File;
use std::io::Read;

impl Config {
	pub fn read_from(filename: &str) ->Config {
		let mut file = File::open(filename)
				.expect("config file not found");
		let mut content = String::new();

		file.read_to_string(&mut content).unwrap();

		let ret: Config = from_str(&content)
				.expect("fail to parse");

		ret 
	}
}
