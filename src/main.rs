extern crate serde;
extern crate toml;

use std::process;

use config::Config;

mod libminiflux;
mod config;
mod ui;


#[tokio::main]
async fn main() {
    let maybe_config_file_path = config::get_config_file_path();
    if maybe_config_file_path.is_none() {
        println!(
            "Cannot find config file directory; this should only happen when there's no home directory for this user" 
        );
        process::exit(1)
    }
    let config_file_path = maybe_config_file_path.as_ref().unwrap();
    let maybe_config = Config::from_file(&config_file_path);
    if maybe_config.is_err() {
        println!(
            "Error parsing config file at {}: {}", 
            maybe_config_file_path.unwrap().to_str().unwrap(), 
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
