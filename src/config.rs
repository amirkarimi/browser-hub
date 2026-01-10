use std::io;

use directories::BaseDirs;
use serde::{Deserialize, Serialize};

const CONFIG_FILE_PATH: &str = "browser-hub/config.json";

#[derive(Debug)]
pub enum ConfigError {
    #[allow(dead_code)]
    IoError(io::Error),
    #[allow(dead_code)]
    JsonError(serde_json::Error),
}

impl From<io::Error> for ConfigError {
    fn from(err: io::Error) -> ConfigError {
        ConfigError::IoError(err)
    }
}

impl From<serde_json::Error> for ConfigError {
    fn from(err: serde_json::Error) -> ConfigError {
        ConfigError::JsonError(err)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct URLTransformer {
    pub keywords: Vec<String>,
    pub from_url_regex: String,
    pub to_url: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Browser {
    pub open_cmd: String,
    #[serde(default)]
    pub process_names: Vec<String>,
    pub cmd_includes_regex: String,
    pub cmd_excludes_regex: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Profile {
    pub name: String,
    pub browser: Browser,
    #[serde(default)]
    pub url_patterns: Vec<String>,
    #[serde(default)]
    pub url_transformers: Vec<URLTransformer>,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Config {
    pub default_browser_open_cmd: String,
    pub profiles: Vec<Profile>,
    // Open these URLs in the profile-specific browser if one is open
    #[serde(default)]
    pub profile_specific_urls: Vec<String>,
}

pub(crate) fn load_config() -> Result<Config, ConfigError> {
    let config_base_path = BaseDirs::new().ok_or_else(|| {
        io::Error::new(io::ErrorKind::NotFound, "Could not determine home directory")
    })?.config_dir().to_path_buf();
    let config_path = config_base_path.join(CONFIG_FILE_PATH);
    let config_data = std::fs::read_to_string(config_path)?;
    let config: Config = serde_json::from_str(&config_data)?;
    Ok(config)
}
