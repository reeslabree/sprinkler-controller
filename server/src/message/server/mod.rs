pub mod controller_heartbeat;

use crate::message::server::controller_heartbeat::ControllerHeartbeatPayload;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type", content = "payload", rename_all = "camelCase")]
pub enum ServerResponse {
    ControllerHeartbeat(ControllerHeartbeatPayload),
}
