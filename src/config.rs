extern crate directories;

use std::{path::PathBuf, fmt::Display, error::Error};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
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

#[derive(Debug, Clone)]
pub struct CannotFindConfigDirError;
impl Display for CannotFindConfigDirError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Couldn't figure out where to put the config file. This should only happen if the user somehow doesn't have a home directory")
    }
}
impl Error for CannotFindConfigDirError {}

#[derive(Debug, Clone)]
pub struct ConfigFileAlreadyExistsError {
    path: String
}
impl Display for ConfigFileAlreadyExistsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Configuration file already exists at {}", self.path)
    }
}
impl Error for ConfigFileAlreadyExistsError {}

pub fn get_config_file_path() -> Result<PathBuf, CannotFindConfigDirError> {
    let path = directories::ProjectDirs::from("com", "spencerwi", "cliflux")
        .map(|project_dirs| {
            let mut config_path = project_dirs.config_dir().to_owned();
            config_path.push(PathBuf::from("config.toml"));
            return config_path;
        });
    match path {
        Some(p) => Ok(p),
        None => Err(CannotFindConfigDirError)
    }
}

pub fn init() -> Result<PathBuf, Box<dyn std::error::Error>> {
    let config_file_path = get_config_file_path()?;
    if config_file_path.exists() {
        return Err(
            Box::new(ConfigFileAlreadyExistsError {
                path: config_file_path.to_str().unwrap().to_string()
            })
        )
    }

    std::fs::create_dir_all(config_file_path.parent().unwrap())?;
    std::fs::write(
        &config_file_path, 
        toml::to_string(&Config {
            api_key: "FIXME".to_string(),
            server_url: "FIXME".to_string()
        })?
    )?;
    return Ok(config_file_path);
}
