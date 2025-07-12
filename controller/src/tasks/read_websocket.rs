use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use esp_hal::gpio::Level;
use heapless::{String, Vec};
use log::{error, info};
use shared::ServerMessage;

use crate::consts::{BUFFER_SIZE, READ_TIMEOUT_MS};
use crate::embassy_websocket::EmbassyWebSocket;
use crate::tasks::toggle_zone;
use crate::types::DioControllerMutex;

#[embassy_executor::task]
pub async fn read_websocket(
    websocket: &'static EmbassyWebSocket<'static>,
    controller: &'static DioControllerMutex,
    spawner: Spawner,
) {
    loop {
        if !websocket.is_connected().await {
            Timer::after(Duration::from_millis(2_500)).await;
            continue;
        }

        let mut buffer = [0; BUFFER_SIZE];

        let message_len = match websocket
            .read_with_timeout(&mut buffer, Duration::from_millis(READ_TIMEOUT_MS))
            .await
        {
            Ok(len) => len,
            Err(_) => {
                Timer::after(Duration::from_millis(50)).await;
                continue;
            }
        };

        if message_len == 0 {
            Timer::after(Duration::from_millis(50)).await;
            continue;
        }

        let vectorized: Vec<u8, BUFFER_SIZE> = match Vec::from_slice(&buffer[2..message_len]) {
            Ok(vec) => vec,
            Err(_) => {
                error!("Failed to vectorize message");
                continue;
            }
        };

        let message: String<BUFFER_SIZE> = match String::from_utf8(vectorized) {
            Ok(string) => string,
            Err(e) => {
                error!("Failed to convert message to string: {:?}", e);
                continue;
            }
        };

        info!("Received message: {}", message);

        let parsed: ServerMessage = match serde_json::from_str(&message) {
            Ok(parsed) => parsed,
            Err(e) => {
                error!("Failed to parse message: {:?}", e);
                continue;
            }
        };

        match parsed {
            ServerMessage::ToggleZone(payload) => {
                info!("Activating zone: {:?}, {}", payload.zone, payload.activate);
                match spawner.spawn(toggle_zone(
                    controller,
                    payload.zone as usize,
                    if payload.activate {
                        Level::High
                    } else {
                        Level::Low
                    },
                )) {
                    Ok(_) => (),
                    Err(e) => error!("Failed to spawn toggle_zone task: {:?}", e),
                };
            }
        }
    }
}
