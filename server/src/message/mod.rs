pub mod server;
pub mod user;

use tokio_tungstenite::tungstenite::Message;

use crate::{
    message::{
        server::ServerResponse,
        user::{
            UserMessage, UserMessageResponse, get_config::GetConfigResponse,
            set_schedule::SetScheduleResponse, status::StatusResponse,
            toggle_zone::ToggleZoneResponse,
        },
    },
    types::{ClientMap, ClientType, ConfigMutex, ControllerTimestamp},
};

use shared::{ControllerMessage, ServerMessage, ToggleZonePayload};

pub async fn send_to_client(clients: &ClientMap, client_type: &ClientType, message: &str) -> bool {
    let clients = clients.lock().await;
    if let Some(sender) = clients.get(client_type) {
        sender.send(Message::Text(message.to_string())).is_ok()
    } else {
        false
    }
}

pub async fn send_to_controller(clients: &ClientMap, message: &str) -> bool {
    send_to_client(clients, &ClientType::Controller, message).await
}

pub async fn handle_user_message(
    clients: &ClientMap,
    controller_timestamp: &ControllerTimestamp,
    config: &ConfigMutex,
    msg: UserMessage,
) {
    println!("User Message: {msg:?}");

    match msg {
        UserMessage::ToggleZone(payload) => {
            send_to_controller(
                clients,
                &serde_json::to_string(&ServerMessage::ToggleZone(ToggleZonePayload {
                    zone: payload.zone,
                    activate: payload.activate,
                }))
                .unwrap(),
            )
            .await;

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
        UserMessage::Status(_payload) => {
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
        UserMessage::KeepAlive(_payload) => {}
        UserMessage::SetSchedule(payload) => {
            let mut config_guard = config.lock().await;
            config_guard.set_schedules(payload.schedules);
            let response = match config_guard.save() {
                Ok(_) => SetScheduleResponse {
                    success: true,
                    error: None,
                },
                Err(e) => SetScheduleResponse {
                    success: false,
                    error: Some(e.to_string()),
                },
            };

            send_to_client(
                clients,
                &ClientType::User,
                &serde_json::to_string(&UserMessageResponse::SetScheduleResponse(response))
                    .unwrap(),
            )
            .await;
        }
        UserMessage::GetConfig(_payload) => {
            let config_guard = config.lock().await;
            let config = config_guard.clone();
            let response = GetConfigResponse {
                schedules: config.schedules,
                stagger_on: config.stagger_on,
                stagger_zones: config.stagger_zones,
            };

            send_to_client(
                clients,
                &ClientType::User,
                &serde_json::to_string(&UserMessageResponse::GetConfigResponse(response)).unwrap(),
            )
            .await;
        }
    }
}

pub async fn handle_controller_message(_clients: &ClientMap, msg: ControllerMessage) {
    match msg {
        ControllerMessage::KeepAlive(_payload) => {}
    }
}

pub async fn handle_server_message(
    clients: &ClientMap,
    recipient: ClientType,
    msg: ServerResponse,
) {
    send_to_client(clients, &recipient, &serde_json::to_string(&msg).unwrap()).await;
}
