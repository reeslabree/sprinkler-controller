pub mod toggle_zone;

use serde::{Deserialize, Serialize};
pub use toggle_zone::{ToggleZonePayload, ToggleZoneResponse};

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type", content = "payload", rename_all = "camelCase")]
pub enum ServerMessage {
    ToggleZone(ToggleZonePayload),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type", content = "payload", rename_all = "camelCase")]
pub enum ServerMessageResponse {
    ToggleZoneResponse(ToggleZoneResponse),
}
