pub mod controller;
pub mod server;
pub mod user;

use tokio_tungstenite::tungstenite::Message;

use crate::{
    message::{
        controller::ControllerMessage,
        server::ServerResponse,
        user::{
            UserMessage, UserMessageResponse, status::StatusResponse,
            toggle_zone::ToggleZoneResponse,
        },
    },
    types::{ClientMap, ClientType, ControllerTimestamp},
};

pub async fn send_to_client(clients: &ClientMap, client_type: &ClientType, message: &str) -> bool {
    let clients = clients.lock().await;
    if let Some(sender) = clients.get(client_type) {
        sender.send(Message::Text(message.to_string())).is_ok()
    } else {
        false
    }
}

pub async fn handle_user_message(
    clients: &ClientMap,
    controller_timestamp: &ControllerTimestamp,
    msg: UserMessage,
) {
    match msg {
        UserMessage::ToggleZone(payload) => {
            println!("ToggleZone: {payload:?}");

            // TODO: write to controller to toggle zone

            send_to_client(
                clients,
                &ClientType::User,
                &serde_json::to_string(&UserMessageResponse::ToggleZoneResponse(
                    ToggleZoneResponse {
                        success: true,
                        error: None,
                    },
                ))
                .unwrap(),
            )
            .await;
        }
        UserMessage::Status(payload) => {
            println!("Status: {payload:?}");

            let is_controller_connected = {
                let timestamp_guard = controller_timestamp.lock().await;
                if let Some(last_message) = *timestamp_guard {
                    last_message.elapsed() < std::time::Duration::from_secs(15)
                } else {
                    false
                }
            };

            send_to_client(
                clients,
                &ClientType::User,
                &serde_json::to_string(&UserMessageResponse::StatusResponse(StatusResponse {
                    is_controller_connected,
                }))
                .unwrap(),
            )
            .await;
        }
        UserMessage::KeepAlive(payload) => {
            println!("KeepAlive: {payload:?}");
        }
    }
}

pub async fn handle_controller_message(clients: &ClientMap, msg: ControllerMessage) {
    match msg {
        _ => (),
    }
}

pub async fn handle_server_message(
    clients: &ClientMap,
    recipient: ClientType,
    msg: ServerResponse,
) {
    send_to_client(clients, &recipient, &serde_json::to_string(&msg).unwrap()).await;
}
