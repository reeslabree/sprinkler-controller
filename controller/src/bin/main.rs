#![no_std]
#![no_main]

use core::{net::Ipv4Addr, str::FromStr};
use embassy_executor::Spawner;
use embassy_net::{
    dns::DnsSocket,
    tcp::{
        client::{TcpClient, TcpClientState},
        TcpSocket,
    },
    Config, Runner, Stack, StackResources,
};
use embassy_time::{Duration, Timer};
use embedded_websocket::{framer::Framer, WebSocket, WebSocketOptions};
use esp_backtrace as _;
use esp_hal::timer::systimer::SystemTimer;
use esp_hal::timer::timg::TimerGroup;
use esp_hal::{clock::CpuClock, rng::Rng};
use esp_wifi::{
    wifi::{ClientConfiguration, Configuration, WifiController, WifiDevice, WifiEvent, WifiState},
    EspWifiController,
};
use log::{error, info};

extern crate alloc;

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

#[esp_hal_embassy::main]
async fn main(spawner: Spawner) {
    esp_println::logger::init_logger_from_env();

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

    spawner.spawn(connection(controller)).ok();
    spawner.spawn(net_task(runner)).ok();
}

#[embassy_executor::task]
async fn connection(mut controller: WifiController<'static>) {
    loop {
        match esp_wifi::wifi::wifi_state() {
            WifiState::StaConnected => {
                controller.wait_for_event(WifiEvent::StaDisconnected).await;
                Timer::after(Duration::from_millis(5000)).await
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
                ..Default::default()
            });
            controller.set_configuration(&client_config).unwrap();
            controller.start_async().await.unwrap();
        }

        match controller.connect_async().await {
            Ok(_) => {
                info!("wifi connected!");
            }
            Err(e) => {
                info!("failed to connect to wifi: {e:?}");
                Timer::after(Duration::from_millis(5000)).await
            }
        }
    }
}

#[embassy_executor::task]
async fn websocket_task(stack: Stack<'static>, rng: Rng) {
    stack.wait_config_up().await;

    loop {
        let mut rx_buffer = [0; 4096];
        let mut tx_buffer = [0; 4096];
        let mut socket = TcpSocket::new(stack, &mut rx_buffer, &mut tx_buffer);

        let remote_endpoint = (
            Ipv4Addr::from_str(WEBSOCKET_IP).unwrap(),
            u16::from_str(WEBSOCKET_PORT).unwrap(),
        );
        match socket.connect(remote_endpoint).await {
            Ok(_) => {
                info!("Connected");
                // let mut websocket = WebSocket::<Rng, embedded_websocket::Client>::new_client(rng);

                // let websocket_options = WebSocketOptions {
                //     path: "/",
                //     host: WEBSOCKET_IP,
                //     origin: "http://localhost",
                //     sub_protocols: None,
                //     additional_headers: None,
                // };

                // let mut read_buf = [0; BUFFER_SIZE];
                // let mut read_cursor = 0;
                // let mut write_buf = [0; BUFFER_SIZE];
                // let mut frame_buf = [0; BUFFER_SIZE];
                // let mut framer = Framer::new(
                //     &mut read_buf,
                //     &mut read_cursor,
                //     &mut write_buf,
                //     &mut websocket,
                // );

                // let mut stream = network::NetworkConnection::new(
                //     socket,
                //     Ipv4Address::new(192, 168, 0, 24),
                //     7000,
                // )
                // .unwrap();
                // framer
                //     .connect(&mut stream, &websocket_options)
                //     .expect("connection error");
            }
            Err(e) => {
                error!("failed to connect to websocket: {e:?}");
                Timer::after(Duration::from_millis(5000)).await;
                continue;
            }
        };
    }
}

#[embassy_executor::task]
async fn net_task(mut runner: Runner<'static, WifiDevice<'static>>) {
    runner.run().await
}
