use crate::types::{ClientMap, ClientType};
use crate::types::{ControllerMessage, UserMessage};
use crate::types::{IsControllerConnectedResponse, ServerResponse};
use tokio_tungstenite::tungstenite::Message;

pub async fn send_to_client(clients: &ClientMap, client_type: &ClientType, message: &str) -> bool {
    let clients = clients.lock().await;
    if let Some(sender) = clients.get(client_type) {
        sender.send(Message::Text(message.to_string())).is_ok()
    } else {
        false
    }
}

pub async fn handle_user_message(clients: &ClientMap, msg: UserMessage) {
    match msg {
        UserMessage::SetSchedule(schedule) => {
            println!("SetSchedule: {schedule:?}");
        }
        UserMessage::IsControllerConnected => {
            let controller_connected = clients.lock().await.get(&ClientType::Controller).is_some();
            send_to_client(
                clients,
                &ClientType::User,
                &format!(
                    "{}",
                    serde_json::to_string(&ServerResponse::IsControllerConnected(
                        IsControllerConnectedResponse {
                            is_connected: controller_connected,
                        },
                    ))
                    .unwrap()
                ),
            )
            .await;
        }
    }
}

pub async fn handle_controller_message(clients: &ClientMap, msg: ControllerMessage) {}
