use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::Display;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::sync::mpsc::UnboundedSender;
use tokio_tungstenite::tungstenite;

pub type ClientMap = Arc<Mutex<HashMap<ClientType, UnboundedSender<tungstenite::Message>>>>;

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

#[derive(Serialize, Deserialize, Debug)]
pub enum Day {
    Monday = 0,
    Tuesday = 1,
    Wednesday = 2,
    Thursday = 3,
    Friday = 4,
    Saturday = 5,
    Sunday = 6,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Schedule {
    pub days: Vec<Day>,
    pub zone: u16,
    pub start_time: String,
    pub end_time: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SetSchedulePayload {
    pub zone_schedules: Vec<Schedule>,
}

// SetSchedule
#[derive(Serialize, Deserialize, Debug)]
pub enum SetScheduleResponseStatus {
    Ok,
    Error,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SetSchedlueResponse {
    pub status: SetScheduleResponseStatus,
    pub message: String,
}

// IsControllerConnected
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct IsControllerConnectedResponse {
    pub is_connected: bool,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type", content = "payload", rename_all = "camelCase")]
pub enum UserMessage {
    SetSchedule(SetSchedulePayload),
    IsControllerConnected,
}

// Controller Events
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum ControllerEventType {
    ZoneTurnedOn,
    ZoneTurnedOff,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ControllerEvent {
    pub event_type: ControllerEventType,
    pub details: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type", content = "payload", rename_all = "camelCase")]
pub enum ControllerMessage {
    SetScheduleResponse(SetSchedlueResponse),
    ControllerEvent(ControllerEvent),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type", content = "payload", rename_all = "camelCase")]
pub enum Message {
    User(UserMessage),
    Controller(ControllerMessage),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type", content = "payload", rename_all = "camelCase")]
pub enum ServerResponse {
    IsControllerConnected(IsControllerConnectedResponse),
}
