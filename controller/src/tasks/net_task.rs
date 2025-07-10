use embassy_net::Runner;
use esp_wifi::wifi::WifiDevice;

#[embassy_executor::task]
pub async fn net_task(mut runner: Runner<'static, WifiDevice<'static>>) {
    runner.run().await
}
