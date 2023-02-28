extern crate directories;

use std::path::PathBuf;

use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub api_key: String,
    pub server_url: String,
}

impl Config {
    pub fn from_file(path: &PathBuf) -> Result<Config, Box<dyn std::error::Error>> {
        let file_contents = std::fs::read_to_string(path)?;
        let parsed_result = toml::from_str::<Config>(&file_contents)?;
        return Ok(parsed_result);
    }
}

pub fn get_config_file_path() -> Option<PathBuf> {
    directories::ProjectDirs::from("com", "spencerwi", "cliflux")
        .map(|project_dirs| {
            let mut config_dir = project_dirs.config_dir().to_owned();
            config_dir.push(PathBuf::from("config.toml"));
            return config_dir;
        })
}
