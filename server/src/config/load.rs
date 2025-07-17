use crate::config::{CONFIG_FILE_PATH, Config};
use std::fs;

pub fn load() -> Result<Config, String> {
    let file = match fs::read_to_string(CONFIG_FILE_PATH) {
        Ok(file) => file,
        Err(e) => match e.kind() {
            std::io::ErrorKind::NotFound => return Ok(Config::default()),
            _ => return Err("Failed to read config file".to_string()),
        },
    };
    let config: Config = toml::from_str(&file).map_err(|_| "Failed to parse config file")?;
    Ok(config)
}
