extern crate serde;
extern crate toml;
extern crate xdg;

use std::{fs, path::Path};

use serde::Deserialize;
use tokio::io::{stderr, AsyncWriteExt};
use tui::Message;
use tuirealm::Update;

mod libminiflux;
mod tui;
mod components;

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
    let mut model = tui::Model::new(miniflux_client);
    let _ = model.terminal.enter_alternate_screen();
    let _ = model.terminal.enable_raw_mode();
    while !model.quit {
        // When RefreshRequested events are processed, a new thread fetches updated entries, and
        // throws them into a tokio channel. We should periodically check that channel to see if 
        // messages have finished fetching, and if so, update the model with them.
        match model.entries_rx.try_recv() {
            Ok(Some(updated_entries)) => {
                model.redraw = true;
                let mut msg = Some(Message::FeedEntriesReceived(updated_entries));
                while msg.is_some() {
                    msg = model.update(msg);
                }
            },
            Err(e) => {
                let _ = stderr().write(format!("{}", e).as_bytes());
            }
            _ => {}
        }
        match model.app.tick(tuirealm::PollStrategy::Once) {
            Err(err) => {
                panic!("{}", err)
            },
            Ok(messages) if messages.len() > 0 => {
                model.redraw = true;
                for msg in messages.into_iter() {
                    let mut msg = Some(msg);
                    while msg.is_some() {
                        msg = model.update(msg)
                    }
                }
            },
            _ => {}
        }
        if model.redraw {
            model.view();
            model.redraw = false;
        }
    }
    let _ = model.terminal.leave_alternate_screen();
    let _ = model.terminal.disable_raw_mode();
}
