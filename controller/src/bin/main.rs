#![no_std]
#![no_main]

use core::net::Ipv4Addr;
use core::str::FromStr;

// WebSocket functionality is now handled by the websocket module
use controller::embassy_websocket::EmbassyWebSocket;
use embassy_executor::Spawner;
use embassy_net::{Runner, Stack, StackResources};
use embassy_time::{Duration, Timer};
use esp_backtrace as _;
use esp_hal::clock::CpuClock;
use esp_hal::timer::systimer::SystemTimer;
use esp_hal::timer::timg::TimerGroup;
use esp_wifi::{
    wifi::{ClientConfiguration, Configuration, WifiController, WifiDevice, WifiEvent, WifiState},
    EspWifiController,
};
use heapless::String;
use log::{info, warn};

macro_rules! mk_static {
    ($t:ty,$val:expr) => {{
        static STATIC_CELL: static_cell::StaticCell<$t> = static_cell::StaticCell::new();
        #[deny(unused_attributes)]
        let x = STATIC_CELL.uninit().write(($val));
        x
    }};
}

const BUFFER_SIZE: usize = 4000;

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

    // Init websocket
    let mut path = String::new();
    let _ = path.push_str(WEBSOCKET_PATH);
    let websocket = mk_static!(
        EmbassyWebSocket<'static>,
        EmbassyWebSocket::new(
            Ipv4Addr::from_str(WEBSOCKET_IP).unwrap(),
            WEBSOCKET_PORT.parse::<u16>().unwrap(),
            path,
            rng,
        )
        .unwrap()
    );

    // Spawn task
    let controller = mk_static!(WifiController<'static>, controller);
    let stack = mk_static!(Stack<'static>, stack);

    spawner.spawn(connection(controller, stack, websocket)).ok();
    spawner.spawn(net_task(runner)).ok();
}

#[embassy_executor::task]
async fn connection(
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

                    let rx_buffer = mk_static!([u8; BUFFER_SIZE], [0; BUFFER_SIZE]);
                    let tx_buffer = mk_static!([u8; BUFFER_SIZE], [0; BUFFER_SIZE]);

                    info!("Connecting to websocket");
                    match websocket.connect(&stack, rx_buffer, tx_buffer).await {
                        Ok(()) => {
                            info!("Connected to websocket");
                            let mut connection_text = String::<10>::new();
                            let _ = connection_text.push_str("controller");
                            websocket.write_text(connection_text).await.unwrap();
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

#[embassy_executor::task]
async fn net_task(mut runner: Runner<'static, WifiDevice<'static>>) {
    runner.run().await
}
