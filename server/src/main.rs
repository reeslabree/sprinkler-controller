mod config;
mod error;
mod message;
mod scheduler_runner;
mod types;

use futures_util::{SinkExt, StreamExt};
use shared::ControllerMessage;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::net::TcpListener;
use tokio::sync::Mutex;
use tokio::sync::mpsc::unbounded_channel;
use tokio_tungstenite::accept_async;

use crate::config::Config;
use crate::message::server::ServerResponse;
use crate::message::server::controller_heartbeat::ControllerHeartbeatPayload;
use crate::message::user::UserMessage;
use crate::message::{
    handle_controller_message, handle_server_message, handle_user_message, send_to_client,
};
use crate::scheduler_runner::ScheduleRunner;
use crate::types::{ClientMap, ClientType, ConfigMutex, ControllerTimestamp, ScheduleRunnerMutex};

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("0.0.0.0:9001").await.unwrap();
    let clients: ClientMap = types::ClientMap::default();
    let controller_timestamp: ControllerTimestamp = Arc::new(Mutex::new(None));
    let config: ConfigMutex = Arc::new(Mutex::new(Config::load().unwrap()));
    let schedule_runner: ScheduleRunnerMutex = Arc::new(Mutex::new(ScheduleRunner::new(
        config.lock().await.clone(),
        &clients,
    )));

    // Spawn heartbeat task
    let heartbeat_clients = clients.clone();
    let heartbeat_timestamp = controller_timestamp.clone();
    tokio::spawn(async move {
        heartbeat_task(heartbeat_clients, heartbeat_timestamp).await;
    });

    while let Ok((stream, _)) = listener.accept().await {
        let clients = clients.clone();
        let controller_timestamp = controller_timestamp.clone();
        let config = config.clone();
        let schedule_runner = schedule_runner.clone();

        tokio::spawn(async move {
            let ws_stream = match accept_async(stream).await {
                Ok(ws) => ws,
                Err(_) => return,
            };
            let (mut write, mut read) = ws_stream.split();

            // get client type
            let Some(Ok(msg)) = read.next().await else {
                return;
            };

            // parse client type
            let client_type = match msg.to_text().unwrap().to_string().as_str() {
                "user" => ClientType::User,
                "controller" => ClientType::Controller,
                _ => return,
            };

            // create and store channel for client
            let (tx, mut rx) = unbounded_channel();
            {
                let mut clients = clients.lock().await;
                if clients.contains_key(&client_type) {
                    clients.remove(&client_type);
                }
                clients.insert(client_type, tx);
            }

            println!("Connected: {client_type}");

            // spawn task in charge of sending messages
            let write_task = tokio::spawn(async move {
                while let Some(msg) = rx.recv().await {
                    if write.send(msg).await.is_err() {
                        break;
                    }
                }
            });

            // spawn task in charge of receiving messages
            while let Some(Ok(msg)) = read.next().await {
                if msg.is_text() {
                    let text = msg.to_text().unwrap();

                    handle_incoming_message(
                        &clients,
                        &controller_timestamp,
                        client_type,
                        text,
                        &config,
                        &schedule_runner,
                    )
                    .await;
                }
            }

            // handle disconnect
            println!("Disconnected: {client_type}");
            {
                let mut clients = clients.lock().await;
                clients.remove(&client_type);
            }
            write_task.abort();
        });
    }
}

async fn heartbeat_task(clients: ClientMap, controller_timestamp: ControllerTimestamp) {
    let mut interval = tokio::time::interval(Duration::from_secs(5)); // Check every 5 seconds

    loop {
        interval.tick().await;

        let is_controller_connected = {
            let timestamp_guard = controller_timestamp.lock().await;
            if let Some(last_message) = *timestamp_guard {
                last_message.elapsed() < Duration::from_secs(15)
            } else {
                false
            }
        };

        if clients.lock().await.contains_key(&ClientType::User) {
            handle_server_message(
                &clients,
                ClientType::User,
                ServerResponse::ControllerHeartbeat(ControllerHeartbeatPayload {
                    is_controller_connected,
                }),
            )
            .await
        }
    }
}

pub async fn handle_incoming_message(
    clients: &ClientMap,
    controller_timestamp: &ControllerTimestamp,
    client_type: ClientType,
    text: &str,
    config: &ConfigMutex,
    schedule_runner: &ScheduleRunnerMutex,
) {
    match client_type {
        ClientType::User => {
            let parsed_msg: UserMessage = match serde_json::from_str(text) {
                Ok(msg) => msg,
                Err(e) => {
                    println!("Error parsing message: {e}");
                    send_to_client(
                        &clients,
                        &ClientType::Controller,
                        &format!("Error parsing user message: {e}"),
                    )
                    .await;
                    return;
                }
            };

            handle_user_message(
                &clients,
                &controller_timestamp,
                &config,
                &schedule_runner,
                parsed_msg,
            )
            .await;
        }
        ClientType::Controller => {
            {
                let mut timestamp_guard = controller_timestamp.lock().await;
                *timestamp_guard = Some(Instant::now());
            }

            let parsed_msg: ControllerMessage = match serde_json::from_str(text) {
                Ok(msg) => msg,
                Err(e) => {
                    println!("Error parsing controller message: {e}");
                    return;
                }
            };

            handle_controller_message(clients, parsed_msg).await;
        }
    }
}
