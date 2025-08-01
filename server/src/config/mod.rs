pub mod load;
pub mod save;

use crate::error::ServerError;
use crate::types::Schedules;

use core::default::Default;
use load::load;
use save::save;
use serde::{Deserialize, Serialize};

pub const CONFIG_FILE_PATH: &str = ".config.toml";

#[derive(Serialize, Deserialize, Clone)]
pub struct Config {
    pub schedules: Schedules,
    pub stagger_on: bool,
    pub stagger_zones: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            schedules: vec![],
            stagger_on: false,
            stagger_zones: false,
        }
    }
}

impl Config {
    pub fn load() -> Result<Self, ServerError> {
        load()
    }

    pub fn save(&self) -> Result<(), ServerError> {
        save(self)
    }

    pub fn set_schedules(&mut self, schedules: Schedules) {
        self.schedules = schedules;
    }

    pub fn set_stagger_on(&mut self, stagger_on: bool) {
        self.stagger_on = stagger_on;
    }

    pub fn set_stagger_zones(&mut self, stagger_zones: bool) {
        self.stagger_zones = stagger_zones;
    }
}
