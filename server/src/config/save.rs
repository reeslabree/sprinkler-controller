use crate::config::{CONFIG_FILE_PATH, Config};
use crate::error::ServerError;

use std::fs;
use std::io::Write;

pub fn save(config: &Config) -> Result<(), ServerError> {
    let toml_string = toml::to_string(config).map_err(|_| ServerError::InvalidTOML)?;
    let mut file = fs::File::create(CONFIG_FILE_PATH)
        .map_err(|_| ServerError::FailedToCreateFile(CONFIG_FILE_PATH.to_string()))?;
    file.write_all(toml_string.as_bytes())
        .map_err(|_| ServerError::FailedToWriteToFile(CONFIG_FILE_PATH.to_string()))?;

    Ok(())
}
