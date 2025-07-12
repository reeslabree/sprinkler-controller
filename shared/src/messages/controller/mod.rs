pub mod keep_alive;

pub use keep_alive::{KeepAlivePayload, KeepAliveResponse};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type", content = "payload", rename_all = "camelCase")]
pub enum ControllerMessage {
    KeepAlive(KeepAlivePayload),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type", content = "payload", rename_all = "camelCase")]
pub enum ControllerMessageResponse {
    KeepAliveResponse(KeepAliveResponse),
}
