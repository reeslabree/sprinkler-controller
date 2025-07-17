use serde::{Deserialize, Serialize};

use crate::types::Schedules;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetConfigPayload {}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetConfigResponse {
    pub schedules: Schedules,
    pub stagger_on: bool,
    pub stagger_zones: bool,
}
