extern crate directories;

use std::{error::Error, fmt::Display, path::PathBuf};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ThemeConfig {
	#[serde(default = "ThemeConfig::default_unread_color")]
	pub unread_color : String,
	#[serde(default = "ThemeConfig::default_read_color")]
	pub read_color : String,
}
impl ThemeConfig {
	pub fn default_unread_color() -> String {
		tuirealm::props::Color::Reset.to_string()
	}
	pub fn default_read_color() -> String {
		tuirealm::props::Color::Gray.to_string()
	}
}

impl Default for ThemeConfig {
	fn default() -> Self {
		ThemeConfig {
			unread_color: ThemeConfig::default_unread_color(),
			read_color: ThemeConfig::default_read_color(),
		}
	}
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub api_key: String,
    pub server_url: String,
    #[serde(default)]
    pub allow_invalid_certs: bool,
    #[serde(default)]
    pub use_rustls: bool,
	#[serde(default)]
	pub theme : ThemeConfig,
}
impl Display for Config {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let stringified = match toml::to_string(self) {
			Ok(s) => s,
			Err(e) => e.to_string()
		};
		write!(f, "{}", stringified)
    }
}

impl Config {
    pub fn from_file(path: &PathBuf) -> Result<Config, Box<dyn std::error::Error>> {
        let file_contents = std::fs::read_to_string(path)?;
        let parsed_result = toml::from_str::<Config>(&file_contents)?;
        let cleaned_server_url = Config::validate_and_clean_server_url(parsed_result.server_url)?;
        return Ok(Config {
            server_url: cleaned_server_url,
            ..parsed_result
        });
    }

    fn validate_and_clean_server_url(url: String) -> Result<String, InvalidServerUrlError> {
        if url.trim().is_empty() {
            return Err(InvalidServerUrlError { value: url });
        }

        if url.ends_with("/") {
            return Ok(url.strip_suffix("/").unwrap().to_string());
        }

        return Ok(url);
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            api_key: "FIXME".to_string(),
            server_url: "FIXME".to_string(),
            allow_invalid_certs: false,
            use_rustls: false,
			theme: ThemeConfig::default()
        }
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
    path: String,
}
impl Display for ConfigFileAlreadyExistsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Configuration file already exists at {}", self.path)
    }
}
impl Error for ConfigFileAlreadyExistsError {}

#[derive(Debug, Clone)]
pub struct InvalidServerUrlError {
    value: String,
}
impl Display for InvalidServerUrlError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Invalid server url: \"{}\"", self.value)
    }
}
impl Error for InvalidServerUrlError {}

pub fn get_config_file_path() -> Result<PathBuf, CannotFindConfigDirError> {
    let path = directories::ProjectDirs::from("com", "spencerwi", "cliflux").map(|project_dirs| {
        let mut config_path = project_dirs.config_dir().to_owned();
        config_path.push(PathBuf::from("config.toml"));
        return config_path;
    });
    match path {
        Some(p) => Ok(p),
        None => Err(CannotFindConfigDirError),
    }
}

pub fn init() -> Result<PathBuf, Box<dyn std::error::Error>> {
    let config_file_path = get_config_file_path()?;
    if config_file_path.exists() {
        return Err(Box::new(ConfigFileAlreadyExistsError {
            path: config_file_path.to_str().unwrap().to_string(),
        }));
    }

    std::fs::create_dir_all(config_file_path.parent().unwrap())?;
    std::fs::write(
        &config_file_path,
        toml::to_string(&Config::default())?,
    )?;
    return Ok(config_file_path);
}
