use crate::embassy_websocket::EmbassyWebSocket;

use embassy_executor::task;
use embassy_time::{Duration, Timer};
use heapless::String;
use log::info;

const KEEP_ALIVE_DURATION_MS: u64 = 10_000;

#[task]
pub async fn keep_alive(websocket: &'static EmbassyWebSocket<'static>) {
    loop {
        Timer::after(Duration::from_millis(KEEP_ALIVE_DURATION_MS)).await;

        if !websocket.is_connected().await {
            info!("Websocket not connected");
            Timer::after(Duration::from_millis(KEEP_ALIVE_DURATION_MS)).await;
            continue;
        }

        let mut keep_alive_packet = String::<33>::new();
        let _ = keep_alive_packet.push_str("{\"type\":\"keepAlive\",\"payload\":{}}");

        info!("Sending keep alive packet");

        websocket.write_text(keep_alive_packet).await.unwrap();
    }
}
