use cyw43_pio::PioSpi;
use embassy_net::{Config as NetConfig, DhcpConfig, IpEndpoint, Stack, StackResources};
use embassy_rp::gpio::Output;
use embassy_rp::peripherals::{DMA_CH0, I2C0, PIN_23, PIN_25, PIO0};

#[embassy_executor::task]
pub async fn wifi_task(runner: cyw43::Runner<'static, Output<'static>, PioSpi<'static, PIO0, 0, DMA_CH0>>) -> ! {
    runner.run().await
}

#[embassy_executor::task]
pub async fn net_task(mut runner: embassy_net::Runner<'static, cyw43::NetDriver<'static>>) -> ! {
    runner.run().await
}
