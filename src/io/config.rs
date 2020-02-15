use super::super::domain;
use std::fs;

const PATH: &str = "config/config.toml";

pub fn load() -> domain::Config {
	let config_string = fs::read_to_string(PATH)
		.unwrap_or_else(|e| panic!(format!("Missing config file at {}: {}", PATH, e)));
	toml::from_str(&config_string).unwrap_or_else(|e| {
		panic!(format!(
			"Error while parsing the config file {}: {}",
			PATH, e
		))
	})
}
