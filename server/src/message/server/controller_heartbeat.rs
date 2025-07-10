use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ControllerHeartbeatPayload {
    pub is_controller_connected: bool,
}
