#![no_std]
#![no_main]

use base64ct::{Base64, Encoding};
use controller::websocket::bytes_to_websocket_frame;
use core::fmt::Write;
use core::{net::Ipv4Addr, str::FromStr};
use embassy_executor::Spawner;
use embassy_net::tcp::TcpSocket;
use embassy_net::{Runner, Stack, StackResources};
use embassy_time::{Duration, Instant, Timer};
use esp_backtrace as _;
use esp_hal::timer::systimer::SystemTimer;
use esp_hal::timer::timg::TimerGroup;
use esp_hal::{clock::CpuClock, rng::Rng};
use esp_wifi::{
    wifi::{ClientConfiguration, Configuration, WifiController, WifiDevice, WifiEvent, WifiState},
    EspWifiController,
};
use heapless::String;
use log::{error, info, warn};

macro_rules! mk_static {
    ($t:ty,$val:expr) => {{
        static STATIC_CELL: static_cell::StaticCell<$t> = static_cell::StaticCell::new();
        #[deny(unused_attributes)]
        let x = STATIC_CELL.uninit().write(($val));
        x
    }};
}

const BUFFER_SIZE: usize = 4000;
const REQUEST_BUFFER_SIZE: usize = 256;
const SOCKET_TIMEOUT_SECONDS: u64 = 20;

const WIFI_SSID: &str = env!("WIFI_SSID");
const WIFI_PASSWORD: &str = env!("WIFI_PASSWORD");
const WEBSOCKET_IP: &str = env!("WEBSOCKET_IP");
const WEBSOCKET_PORT: &str = env!("WEBSOCKET_PORT");
const WEBSOCKET_PATH: &str = env!("WEBSOCKET_PATH");

#[esp_hal_embassy::main]
async fn main(spawner: Spawner) {
    esp_println::logger::init_logger_from_env();

    info!("SSID: {}", WIFI_SSID);
    info!("Password: {}", WIFI_PASSWORD);

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    esp_alloc::heap_allocator!(size: 72 * 1024);

    let timer0 = SystemTimer::new(peripherals.SYSTIMER);
    esp_hal_embassy::init(timer0.alarm0);

    info!("Embassy initialized!");

    let mut rng = esp_hal::rng::Rng::new(peripherals.RNG);

    let timer1 = TimerGroup::new(peripherals.TIMG0);
    let esp_wifi_ctrl: &EspWifiController<'static> = &*mk_static!(
        EspWifiController<'static>,
        esp_wifi::init(timer1.timer0, rng.clone(), peripherals.RADIO_CLK).unwrap()
    );

    let (mut controller, interfaces) =
        esp_wifi::wifi::new(&esp_wifi_ctrl, peripherals.WIFI).unwrap();

    let wifi_interface = interfaces.sta;

    info!("MAC: {:?}", wifi_interface.mac_address());

    controller
        .set_power_saving(esp_wifi::config::PowerSaveMode::None)
        .unwrap();

    let config = embassy_net::Config::dhcpv4(Default::default());

    let seed = (rng.random() as u64) << 32 | rng.random() as u64;

    // Init network stack
    let (stack, runner) = embassy_net::new(
        wifi_interface,
        config,
        mk_static!(StackResources<3>, StackResources::<3>::new()),
        seed,
    );

    spawner.spawn(connection(controller, stack, rng)).ok();

    spawner.spawn(net_task(runner)).ok();
}

#[embassy_executor::task]
async fn connection(mut controller: WifiController<'static>, stack: Stack<'static>, mut rng: Rng) {
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
                channel: Some(1),
                ..Default::default()
            });
            controller.set_configuration(&client_config).unwrap();
            controller.start_async().await.unwrap();
        }

        let (aps, num_aps) = controller.scan_n_async::<3>().await.unwrap();
        if num_aps > 0 {
            info!(" -------------------------------------- ");
            info!("| SSID               | Signal Strength |");
            info!("|--------------------+-----------------|");
            for ap in aps.iter() {
                let ssid_str = ap.ssid.as_str();
                let truncated_ssid = if ssid_str.len() > 18 {
                    &ssid_str[..18]
                } else {
                    ssid_str
                };
                info!("| {:<18} | {:<15} |", truncated_ssid, ap.signal_strength);
            }
            info!(" -------------------------------------- ");
        }

        if aps.iter().any(|ap| ap.ssid.as_str() == WIFI_SSID) {
            info!("Found target WiFi network: {}", WIFI_SSID);
            if let Some(target_ap) = aps.iter().find(|ap| ap.ssid.as_str() == WIFI_SSID) {
                info!(
                    "Target network - Channel: {}, Signal: {}, Auth: {:?}",
                    target_ap.channel, target_ap.signal_strength, target_ap.auth_method
                );
            }
        }

        match controller.connect_async().await {
            Ok(()) => {
                info!("Wifi connected");

                // Wait longer for DHCP to complete
                Timer::after(Duration::from_millis(5000)).await;

                // Try to get network config multiple times
                loop {
                    if controller.is_connected().is_err() || !controller.is_connected().unwrap() {
                        warn!("Wifi disconnected");
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

                if controller.is_connected().is_err() || !controller.is_connected().unwrap() {
                    continue;
                }

                // Get network interface info
                if let Some(config) = stack.config_v4() {
                    info!("IP: {}", config.address);
                    info!("Gateway: {:?}", config.gateway.unwrap());
                    info!("DNS: {}", config.dns_servers[0]);
                } else {
                    warn!("DHCP timed out");
                    controller.disconnect().unwrap();
                    continue;
                }

                let mut rx_buffer: [u8; BUFFER_SIZE] = [0; BUFFER_SIZE];
                let mut tx_buffer: [u8; BUFFER_SIZE] = [0; BUFFER_SIZE];
                let mut socket = TcpSocket::new(stack, &mut rx_buffer, &mut tx_buffer);

                let ip = Ipv4Addr::from_str(WEBSOCKET_IP).unwrap();
                let port = WEBSOCKET_PORT.parse().unwrap();

                info!("Socket connecting to {}:{}", ip, port);

                let connect_start_time = Instant::now();
                let connect_res = socket.connect((ip, port)).await;
                if connect_res.is_err() {
                    error!(
                        "Unable to connect to {}:{} - {:?}",
                        ip,
                        port,
                        connect_res.err()
                    );
                    return;
                }
                info!("Connected to websocket successfully");
                let connect_duration = connect_start_time.elapsed();
                info!("Connection time: {:?}ms", connect_duration.as_millis());

                // compute websocket secret
                let mut random_buffer: [u8; 16] = [0; 16];
                rng.read(&mut random_buffer);

                let mut encode_buffer: [u8; 24] = [0; 24];
                let random_str = match Base64::encode(&random_buffer, &mut encode_buffer) {
                    Ok(t) => t,
                    Err(_) => {
                        error!("Failed encoding base64 websocket-key");
                        return;
                    }
                };
                info!("Base64 encoded websocket-key: {}", random_str);

                // build initial upgrade request
                let mut upgrade_request: String<REQUEST_BUFFER_SIZE> = String::new();

                match write!(
                    &mut upgrade_request,
                    "GET {} HTTP/1.1\r\nHost: {}:{}\r\nUpgrade: websocket\r\nConnection: Upgrade\r\nSec-WebSocket-Key: {}\r\nSec-WebSocket-Version: 13\r\n\r\n",
                    WEBSOCKET_PATH,
                    ip,
                    port,
                    random_str,
                ) {
                    Ok(_) => (),
                    Err(e) => {
                        error!("Failed to write upgrade request: {:?}", e);
                        return;
                    }
                };

                info!("Upgrade request: {}", upgrade_request);

                match socket.write(upgrade_request.as_bytes()).await {
                    Ok(len) => {
                        info!("Upgrade request sent: {} bytes", len);
                    }
                    Err(e) => {
                        error!("Failed upgrade request: {:?}", e);
                        return;
                    }
                };

                // Flush the socket to ensure all data is sent
                if let Err(e) = socket.flush().await {
                    error!("Failed to flush socket: {:?}", e);
                    return;
                }
                info!("Socket flushed, waiting for response...");

                let deadline = Instant::now() + Duration::from_secs(SOCKET_TIMEOUT_SECONDS);
                let mut buffer = [0u8; 512];
                let mut total_bytes = 0;

                while Instant::now() < deadline {
                    match socket.read(&mut buffer).await {
                        Ok(len) => {
                            if len > 0 {
                                total_bytes += len;
                                let to_print =
                                    unsafe { core::str::from_utf8_unchecked(&buffer[..len]) };
                                info!("Received {} bytes:\n{}", len, to_print);

                                if to_print.contains("Sec-WebSocket-Accept")
                                    || to_print.contains("sec-websocket-accept")
                                {
                                    let mut masking_key = [0; 4];
                                    rng.read(&mut masking_key);
                                    let frame = match bytes_to_websocket_frame(
                                        b"controller",
                                        masking_key,
                                    ) {
                                        Ok(frame) => frame,
                                        Err(e) => {
                                            error!(
                                                "Failed to convert bytes to websocket frame: {:?}",
                                                e
                                            );
                                            return;
                                        }
                                    };
                                    info!("Sending controller message");
                                    match socket.write(&frame).await {
                                        Ok(_) => {
                                            socket.flush().await.unwrap();
                                            info!("Sent controller message");
                                        }
                                        Err(e) => {
                                            error!("Failed to write to socket: {:?}", e);
                                            return;
                                        }
                                    };
                                    break;
                                }
                            } else {
                                info!("Socket returned 0 bytes, connection might be closed");
                                break;
                            }
                        }
                        Err(e) => {
                            error!("Error reading from socket: {:?}", e);
                            break;
                        }
                    }
                }

                if total_bytes == 0 {
                    error!("No response received from server within timeout");
                } else {
                    info!("Total bytes received: {}", total_bytes);
                }
            }
            Err(e) => {
                warn!("failed to connect to wifi: {e:?}");
                Timer::after(Duration::from_millis(5000)).await
            }
        }
    }
}

#[embassy_executor::task]
async fn net_task(mut runner: Runner<'static, WifiDevice<'static>>) {
    runner.run().await
}
