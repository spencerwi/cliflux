extern crate serde;
extern crate toml;
extern crate xdg;

use std::{fs, process};

use serde::Deserialize;
use xdg::BaseDirectoriesError;

mod libminiflux;
mod ui;

#[derive(Deserialize, Debug)]
struct Config {
    api_key: String,
    server_url: String,
}

impl Config {
    pub fn from_file(path: &str) -> Result<Config, Box<dyn std::error::Error>> {
        let file_contents = fs::read_to_string(path)?;
        let parsed_result = toml::from_str::<Config>(&file_contents)?;
        return Ok(parsed_result);
    }
}

fn get_config_file_path() -> Result<String, BaseDirectoriesError> {
    let basedirs = xdg::BaseDirectories::new()?;
    return Ok(
        basedirs.get_config_file("cliflux/config.toml")
            .to_str()
            .unwrap()
            .to_string()
    );
}

#[tokio::main]
async fn main() {
    let maybe_config_file_path = get_config_file_path();
    if maybe_config_file_path.is_err() {
        println!(
            "Cannot find config file directory: {}", 
            maybe_config_file_path.unwrap_err()
        );
        process::exit(1)
    }
    let config_file_path = maybe_config_file_path.as_ref().unwrap();
    let maybe_config = Config::from_file(&config_file_path);
    if maybe_config.is_err() {
        println!(
            "Error parsing config file at {}: {}", 
            maybe_config_file_path.unwrap(), 
            maybe_config.unwrap_err()
        );
        process::exit(1)
    }
    let config = maybe_config.unwrap();
    let miniflux_client = libminiflux::Client::new(
        config.server_url.to_string(), 
        &config.api_key
    );
    let mut ui = ui::Ui::new(miniflux_client);
    ui.run()
}
