use crate::config::{CONFIG_FILE_PATH, Config};
use crate::error::ServerError;

use std::fs;

pub fn load() -> Result<Config, ServerError> {
    let file = match fs::read_to_string(CONFIG_FILE_PATH) {
        Ok(file) => file,
        Err(e) => match e.kind() {
            std::io::ErrorKind::NotFound => return Ok(Config::default()),
            _ => return Err(ServerError::InvalidConfig),
        },
    };
    let config: Config = toml::from_str(&file).map_err(|_| ServerError::InvalidConfig)?;
    Ok(config)
}
