pub mod status;
pub mod toggle_zone;

use crate::message::user::status::{StatusPayload, StatusResponse};
use crate::message::user::toggle_zone::{ToggleZonePayload, ToggleZoneResponse};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type", content = "payload", rename_all = "camelCase")]
pub enum UserMessage {
    ToggleZone(ToggleZonePayload),
    Status(StatusPayload),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type", content = "payload", rename_all = "camelCase")]
pub enum UserMessageResponse {
    ToggleZoneResponse(ToggleZoneResponse),
    StatusResponse(StatusResponse),
}
