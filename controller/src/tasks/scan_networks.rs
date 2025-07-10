use embassy_time::{Duration, Timer};
use esp_backtrace as _;
use esp_wifi::wifi::{
    ClientConfiguration, Configuration, Protocol, WifiController, WifiEvent, WifiState,
};
use log::info;

const WIFI_SSID: &str = env!("WIFI_SSID");
const WIFI_PASSWORD: &str = env!("WIFI_PASSWORD");

#[embassy_executor::task]
pub async fn scan_networks(controller: &'static mut WifiController<'static>) {
    loop {
        match esp_wifi::wifi::wifi_state() {
            WifiState::StaConnected => {
                controller.wait_for_event(WifiEvent::StaDisconnected).await;
                Timer::after(Duration::from_millis(5000)).await;
            }
            _ => {}
        }

        if !matches!(controller.is_started(), Ok(true)) {
            let mut ssid = heapless::String::<32>::new();
            let _ = ssid.push_str(WIFI_SSID);

            let mut password = heapless::String::<64>::new();
            let _ = password.push_str(WIFI_PASSWORD);

            let client_config = Configuration::Client(ClientConfiguration {
                ssid: ssid,
                password: password,
                auth_method: esp_wifi::wifi::AuthMethod::WPA2Personal,
                ..Default::default()
            });
            controller.set_configuration(&client_config).unwrap();
            info!("Starting wifi");
            controller.start_async().await.unwrap();
            info!("Wifi started");

            controller.set_protocol(Protocol::P802D11LR.into()).unwrap();
        }

        let (aps, num_aps) = controller.scan_n_async::<3>().await.unwrap();
        if num_aps > 0 {
            info!(" ------------------------------------------------ ");
            info!("| SSID               | Signal Strength | Channel |");
            info!("|--------------------+-----------------+---------|");
            for ap in aps.iter() {
                let ssid_str = ap.ssid.as_str();
                let truncated_ssid = if ssid_str.len() > 18 {
                    &ssid_str[..18]
                } else {
                    ssid_str
                };
                info!(
                    "| {:<18} | {:<15} | {:<7} |",
                    truncated_ssid, ap.signal_strength, ap.channel,
                );
            }
            info!(" ------------------------------------------------ ");
        } else {
            info!("No networks found");
        }

        Timer::after(Duration::from_millis(1000)).await;
    }
}
