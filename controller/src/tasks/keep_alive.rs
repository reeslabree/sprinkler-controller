use crate::consts::KEEP_ALIVE_DURATION_MS;
use crate::embassy_websocket::EmbassyWebSocket;
use embassy_time::{Duration, Timer};
use heapless::String;
use log::{info, warn};
use shared::{ControllerMessage, KeepAlivePayload};

#[embassy_executor::task]
pub async fn keep_alive(websocket: &'static EmbassyWebSocket<'static>) {
    loop {
        Timer::after(Duration::from_millis(KEEP_ALIVE_DURATION_MS)).await;

        if !websocket.is_connected().await {
            info!("Websocket not connected");
            Timer::after(Duration::from_millis(KEEP_ALIVE_DURATION_MS)).await;
            continue;
        }

        let mut keep_alive_packet = String::<33>::new();
        let payload = ControllerMessage::KeepAlive(KeepAlivePayload {});

        let _ = keep_alive_packet.push_str(serde_json::to_string(&payload).unwrap().as_str());

        info!("Sending keep alive packet: {:?}", keep_alive_packet);

        match websocket.write_text(keep_alive_packet).await {
            Ok(()) => {}
            Err(e) => {
                warn!("Failed to send keep alive packet: {:?}", e);
            }
        }
    }
}
