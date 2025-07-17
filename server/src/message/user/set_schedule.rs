use serde::{Deserialize, Serialize};

use crate::types::Schedules;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SetSchedulePayload {
    pub schedules: Schedules,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SetScheduleResponse {
    pub success: bool,
    pub error: Option<String>,
}
