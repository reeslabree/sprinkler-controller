use crate::config::{CONFIG_FILE_PATH, Config};
use std::fs;
use std::io::Write;

pub fn save(config: &Config) -> Result<(), &str> {
    let toml_string = toml::to_string(config).map_err(|_| "Failed to convert config to TOML")?;
    let mut file =
        fs::File::create(CONFIG_FILE_PATH).map_err(|_| "Failed to create config file")?;
    file.write_all(toml_string.as_bytes())
        .map_err(|_| "Failed to write config to file")?;

    Ok(())
}
