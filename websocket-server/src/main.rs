mod message;
mod types;

use crate::message::{handle_user_message, send_to_client};
use crate::types::{ClientMap, ClientType};
use crate::types::{ControllerMessage, UserMessage};
use futures_util::{SinkExt, StreamExt};
use tokio::net::TcpListener;
use tokio::sync::mpsc::unbounded_channel;
use tokio_tungstenite::{accept_async, tungstenite::Message};

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("0.0.0.0:9001").await.unwrap();
    let clients: ClientMap = types::ClientMap::default();

    while let Ok((stream, _)) = listener.accept().await {
        let clients = clients.clone();
        tokio::spawn(async move {
            let ws_stream = match accept_async(stream).await {
                Ok(ws) => ws,
                Err(_) => return,
            };
            let (mut write, mut read) = ws_stream.split();

            let Some(Ok(msg)) = read.next().await else {
                return;
            };

            let client_type = match msg.to_text().unwrap().to_string().as_str() {
                "user" => ClientType::User,
                "controller" => ClientType::Controller,
                _ => return,
            };

            {
                let clients = clients.lock().await;
                if clients.contains_key(&client_type) {
                    let _ = write
                        .send(Message::Text(format!(
                            "Error: {} already connected",
                            client_type
                        )))
                        .await;
                    return;
                }
            }

            let (tx, mut rx) = unbounded_channel();
            clients.lock().await.insert(client_type, tx);

            println!("Connected: {client_type}");

            let write_task = tokio::spawn(async move {
                while let Some(msg) = rx.recv().await {
                    if write.send(msg).await.is_err() {
                        break;
                    }
                }
            });

            while let Some(Ok(msg)) = read.next().await {
                if msg.is_text() {
                    let text = msg.to_text().unwrap();

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
                                    continue;
                                }
                            };

                            println!("User Message: {parsed_msg:?}");

                            handle_user_message(&clients, parsed_msg).await;
                        }
                        ClientType::Controller => {
                            let parsed_msg: ControllerMessage = match serde_json::from_str(text) {
                                Ok(msg) => msg,
                                Err(e) => {
                                    println!("Error parsing controller message: {e}");
                                    continue;
                                }
                            };

                            println!("Controller Message: {parsed_msg:?}");
                        }
                    }
                }
            }

            println!("Disconnected: {client_type}");
            clients.lock().await.remove(&client_type);
            write_task.abort();
        });
    }
}
