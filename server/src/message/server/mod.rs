use serde::{Deserialize, Serialize};

// IsControllerConnected
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(super) struct IsControllerConnectedResponse {
    pub is_connected: bool,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type", content = "payload", rename_all = "camelCase")]
pub(super) enum ServerResponse {
    IsControllerConnected(IsControllerConnectedResponse),
}
