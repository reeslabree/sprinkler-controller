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
