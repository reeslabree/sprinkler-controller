use crate::config::Config;
use crate::scheduler_runner::ScheduleRunner;

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::fmt::{Debug, Display};
use std::sync::Arc;
use std::time::Instant;
use tokio::sync::Mutex;
use tokio::sync::mpsc::UnboundedSender;
use tokio_tungstenite::tungstenite;

pub type ClientMap = Arc<Mutex<HashMap<ClientType, UnboundedSender<tungstenite::Message>>>>;
pub type ControllerTimestamp = Arc<Mutex<Option<Instant>>>;
pub type ConfigMutex = Arc<Mutex<Config>>;
pub type ScheduleRunnerMutex = Arc<Mutex<ScheduleRunner>>;

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Hash, Clone, Copy)]
pub enum ClientType {
    User = 0,
    Controller = 1,
}

impl Display for ClientType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ClientType::User => write!(f, "User"),
            ClientType::Controller => write!(f, "Controller"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Hash, Clone, Copy)]
#[serde(rename_all = "camelCase")]
pub enum Day {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

impl From<chrono::Weekday> for Day {
    fn from(weekday: chrono::Weekday) -> Self {
        match weekday {
            chrono::Weekday::Mon => Day::Monday,
            chrono::Weekday::Tue => Day::Tuesday,
            chrono::Weekday::Wed => Day::Wednesday,
            chrono::Weekday::Thu => Day::Thursday,
            chrono::Weekday::Fri => Day::Friday,
            chrono::Weekday::Sat => Day::Saturday,
            chrono::Weekday::Sun => Day::Sunday,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone, Hash, Copy, Ord, PartialOrd)]
#[serde(rename_all = "camelCase")]
pub enum Zone {
    Zone1 = 0,
    Zone2 = 1,
    Zone3 = 2,
    Zone4 = 3,
    Zone5 = 4,
    Zone6 = 5,
}

impl From<Zone> for u8 {
    fn from(zone: Zone) -> Self {
        zone as u8
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
#[serde(rename_all = "camelCase")]
pub struct ActivePeriod {
    pub zone: Zone,
    pub duration_minutes: u32,
}

// in service of having unique zone entries within the active_periods set
impl PartialEq for ActivePeriod {
    fn eq(&self, other: &Self) -> bool {
        self.zone == other.zone
    }
}

impl Eq for ActivePeriod {}

impl std::hash::Hash for ActivePeriod {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.zone.hash(state);
    }
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Schedule {
    pub name: String,
    pub days: HashSet<Day>,
    pub active_periods: HashSet<ActivePeriod>,
    pub start_time_minutes: u32,
    pub is_active: bool,
}

pub type Schedules = Vec<Schedule>;
