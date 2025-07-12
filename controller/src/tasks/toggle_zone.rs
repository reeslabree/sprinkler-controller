use crate::types::DioControllerMutex;
use esp_hal::gpio::Level;
use log::error;

#[embassy_executor::task]
pub async fn toggle_zone(controller: &'static DioControllerMutex, zone: usize, level: Level) {
    match controller.lock().await.toggle_zone(zone, level) {
        Ok(()) => (),
        Err(e) => error!("Failed to toggle zone {}: {}", zone, e),
    };
}
