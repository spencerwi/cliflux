extern crate serde;
extern crate toml;

use std::{env, process};

use config::Config;

mod config;
mod libminiflux;
mod ui;

pub fn init_config_and_exit() {
    match config::init() {
        Ok(config_path) => {
            println!(
                "Wrote default configuration file to {}",
                config_path.to_str().unwrap()
            );
            process::exit(0);
        }
        Err(e) => {
            eprintln!("Error writing default config file: {}", e);
            process::exit(1);
        }
    }
}

fn print_config_and_exit() {
	let config = read_config();
	println!("{}", config);
	process::exit(0);
}

pub fn print_help_and_exit() {
    println!("USAGE: cliflux [--init|--help|--check-config]");
    process::exit(0);
}

fn has_argument(arg: &str) -> bool {
    env::args().into_iter().any(|a| a.to_lowercase() == arg)
}

fn read_config() -> Config {
    let maybe_config_file_path = config::get_config_file_path();
    if maybe_config_file_path.is_err() {
        eprintln!("{}", maybe_config_file_path.unwrap_err());
        process::exit(1)
    }
    let config_file_path = maybe_config_file_path.unwrap();

    let maybe_config = Config::from_file(&config_file_path);
    if maybe_config.is_err() {
        eprintln!(
            "Error parsing config file at {}: {}",
            &config_file_path.to_str().unwrap(),
            maybe_config.unwrap_err()
        );
        process::exit(1)
    }
    maybe_config.unwrap()
}

#[tokio::main]
async fn main() {
    if has_argument("--help") {
        print_help_and_exit();
    }
    if has_argument("--init") {
        init_config_and_exit()
    }

	if has_argument("--check-config") {
		print_config_and_exit()
	}

	let config = read_config();

    let miniflux_client = libminiflux::Client::new(&config);
    let mut ui = ui::Ui::new(miniflux_client, config.theme);
    ui.run()
}
