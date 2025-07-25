use crate::consts::{BUFFER_SIZE, WIFI_PASSWORD, WIFI_SSID};
use crate::embassy_websocket::EmbassyWebSocket;
use embassy_net::Stack;
use embassy_time::{Duration, Timer};
use esp_backtrace as _;
use esp_wifi::wifi::{ClientConfiguration, Configuration, WifiController, WifiEvent, WifiState};
use heapless::String;
use log::{info, warn};

static mut RX_BUFFER: [u8; BUFFER_SIZE] = [0; BUFFER_SIZE];
static mut TX_BUFFER: [u8; BUFFER_SIZE] = [0; BUFFER_SIZE];

#[embassy_executor::task]
pub async fn connection(
    controller: &'static mut WifiController<'static>,
    stack: &'static Stack<'static>,
    websocket: &'static EmbassyWebSocket<'static>,
) {
    loop {
        match esp_wifi::wifi::wifi_state() {
            WifiState::StaConnected => {
                controller.wait_for_event(WifiEvent::StaDisconnected).await;
                Timer::after(Duration::from_millis(5000)).await;
            }
            _ => {}
        }
        let mut ssid = heapless::String::<32>::new();
        let _ = ssid.push_str(WIFI_SSID);

        let mut password = heapless::String::<64>::new();
        let _ = password.push_str(WIFI_PASSWORD);

        if !matches!(controller.is_started(), Ok(true)) {
            let client_config = Configuration::Client(ClientConfiguration {
                ssid: ssid,
                password: password,
                auth_method: esp_wifi::wifi::AuthMethod::WPA2Personal,
                ..Default::default()
            });
            controller.set_configuration(&client_config).unwrap();
            controller.start_async().await.unwrap();
        }

        match controller.connect_async().await {
            Ok(()) => {
                info!("Wifi connected");

                // Wait longer for DHCP to complete
                Timer::after(Duration::from_millis(5000)).await;

                // Try to get network config multiple times
                let mut should_continue = false;
                loop {
                    if controller.is_connected().is_err() || !controller.is_connected().unwrap() {
                        warn!("Wifi disconnected");
                        should_continue = true;
                        break;
                    }

                    let config = stack.config_v4();
                    if config.is_some() {
                        info!("DHCP completed");
                        break;
                    }
                    info!("Waiting for DHCP...");
                    Timer::after(Duration::from_millis(6000)).await;
                }

                if should_continue {
                    continue;
                }

                // Get network interface info
                if let Some(config) = stack.config_v4() {
                    info!("IP: {}", config.address);
                    info!("Gateway: {:?}", config.gateway.unwrap());
                    info!("DNS: {}", config.dns_servers[0]);

                    let rx_buffer = unsafe { &mut RX_BUFFER };
                    let tx_buffer = unsafe { &mut TX_BUFFER };

                    info!("Connecting to websocket");
                    match websocket.connect(&stack, rx_buffer, tx_buffer).await {
                        Ok(()) => {
                            info!("Connected to websocket");
                            let mut connection_text = String::<10>::new();
                            let _ = connection_text.push_str("controller");
                            match websocket.write_text(connection_text).await {
                                Ok(()) => {
                                    info!("Sent controller identification");
                                }
                                Err(e) => {
                                    warn!("Failed to send controller identification: {:?}", e);
                                }
                            }
                        }
                        Err(e) => {
                            warn!("failed to connect to websocket: {e:?}");
                            Timer::after(Duration::from_millis(5000)).await;
                        }
                    }
                } else {
                    warn!("DHCP timed out");
                    controller.disconnect().unwrap();
                    continue;
                }
            }
            Err(e) => {
                warn!("failed to connect to wifi: {e:?}");
                Timer::after(Duration::from_millis(5000)).await
            }
        }
    }
}
