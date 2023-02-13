extern crate serde;
extern crate toml;
extern crate xdg;

use std::{fs, path::Path};

use serde::Deserialize;

mod libminiflux;
mod tui;

#[derive(Deserialize)]
struct Config {
    api_key: String,
    server_url: String,
}

impl Config {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Config {
        let file_contents = fs::read_to_string(path).unwrap();
        return toml::from_str::<Config>(&file_contents).unwrap();
    }
}

fn get_config_file_path() -> String {
    let basedirs = xdg::BaseDirectories::new().unwrap();
    return basedirs
        .get_config_file("cliflux/config.toml")
        .to_str()
        .unwrap()
        .to_string();
}

#[tokio::main]
async fn main() {
    let config = Config::from_file(get_config_file_path());
    let miniflux_client = libminiflux::Client::new(config.server_url.to_string(), &config.api_key);
    let mut app = tui::Controller::new(miniflux_client);
    app.start().await;
}
