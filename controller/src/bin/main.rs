#![no_std]
#![no_main]

use core::net::Ipv4Addr;
use core::str::FromStr;

use controller::consts::{WEBSOCKET_IP, WEBSOCKET_PATH, WEBSOCKET_PORT};
use controller::dio_controller::DioController;
use controller::embassy_websocket::EmbassyWebSocket;
use controller::macros::mk_static;
use controller::tasks::{connection, keep_alive, net_task, read_websocket};
use controller::types::DioControllerMutex;
use embassy_executor::Spawner;
use embassy_net::{Stack, StackResources};
use embassy_sync::mutex::Mutex;
use esp_backtrace as _;
use esp_hal::clock::CpuClock;
use esp_hal::gpio::AnyPin;
use esp_hal::timer::systimer::SystemTimer;
use esp_hal::timer::timg::TimerGroup;
use esp_wifi::wifi::WifiController;
use esp_wifi::EspWifiController;
use heapless::String;
use log::info;

#[esp_hal_embassy::main]
async fn main(spawner: Spawner) {
    esp_println::logger::init_logger_from_env();

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    // intialize gpio pins
    let zone_pins: [AnyPin; 6] = [
        peripherals.GPIO23.into(),
        peripherals.GPIO22.into(),
        peripherals.GPIO21.into(),
        peripherals.GPIO20.into(),
        peripherals.GPIO19.into(),
        peripherals.GPIO18.into(),
    ];
    let controller = DioController::new(zone_pins);
    let controller_mutex = mk_static!(DioControllerMutex, Mutex::new(controller));

    esp_alloc::heap_allocator!(size: 72 * 1024);

    let timer0 = SystemTimer::new(peripherals.SYSTIMER);
    esp_hal_embassy::init(timer0.alarm0);

    let mut rng = esp_hal::rng::Rng::new(peripherals.RNG);

    let timer1 = TimerGroup::new(peripherals.TIMG0);
    let esp_wifi_ctrl: &EspWifiController<'static> = &*mk_static!(
        EspWifiController<'static>,
        esp_wifi::init(timer1.timer0, rng.clone(), peripherals.RADIO_CLK).unwrap()
    );

    let (mut controller, interfaces) =
        esp_wifi::wifi::new(&esp_wifi_ctrl, peripherals.WIFI).unwrap();

    let wifi_interface = interfaces.sta;

    info!("MAC: {:02X?}", wifi_interface.mac_address());

    controller
        .set_power_saving(esp_wifi::config::PowerSaveMode::None)
        .unwrap();

    let config = embassy_net::Config::dhcpv4(Default::default());

    let seed = (rng.random() as u64) << 32 | rng.random() as u64;

    let (stack, runner) = embassy_net::new(
        wifi_interface,
        config,
        mk_static!(StackResources<3>, StackResources::<3>::new()),
        seed,
    );

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

    let controller = mk_static!(WifiController<'static>, controller);
    let stack = mk_static!(Stack<'static>, stack);

    spawner.spawn(connection(controller, stack, websocket)).ok();
    spawner.spawn(net_task(runner)).ok();
    spawner.spawn(keep_alive(websocket)).ok();
    spawner
        .spawn(read_websocket(websocket, controller_mutex, spawner))
        .ok();
}
