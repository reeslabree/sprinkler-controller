use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type", content = "payload", rename_all = "camelCase")]
pub enum ControllerMessage {}
