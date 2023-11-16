use std::{path::PathBuf, sync::Mutex};

use cli_log::debug;
use config_file::FromConfigFile;
use once_cell::sync::OnceCell;
use serde::Deserialize;

use crate::NgxLaserError;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub base_dir: PathBuf,
    // pub sizes: Vec<String>,
}

impl Config {
    pub fn base_dir(&self) -> &PathBuf {
        &self.base_dir
    }
    // pub fn sizes(&self) -> &Vec<String> {
    //     &self.sizes
    // }
}
// Create a static OnceCell to hold the global configuration
static GLOBAL_CONFIG: OnceCell<Mutex<Config>> = OnceCell::new();

// Implement a function to initialize configuration
pub fn parse_config(path_to_conf: PathBuf) -> Result<Config, NgxLaserError> {
    debug!("INIT config");
    if !path_to_conf.is_file() {
        return Err(NgxLaserError::InvalidConfig);
    }
    debug!("file_exist");
    Config::from_config_file(path_to_conf).or(Err(NgxLaserError::InvalidConfig))
}

pub fn init(conf: Config) -> &'static Mutex<Config> {
    GLOBAL_CONFIG.get_or_init(|| Mutex::new(conf))
}

// Function to get the global configuration
pub fn get() -> &'static Mutex<Config> {
    GLOBAL_CONFIG.get().unwrap()
}
