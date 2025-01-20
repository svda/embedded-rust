use core::str::FromStr;
use cyw43_pio::PioSpi;
use defmt::*;
use embassy_executor::Spawner;
use embassy_net::{Config as NetConfig, DhcpConfig, StackResources};
use embassy_rp::{
    clocks::RoscRng,
    gpio::{Level, Output},
    peripherals::{self, DMA_CH0, PIO0},
    pio::Pio,
    rtc::{DateTime, Rtc},
};
use embassy_time::{Duration, Timer};
use rand_core::RngCore;
use static_cell::StaticCell;

use crate::system::resources::{WifiResources, Irqs};

#[embassy_executor::task]
async fn wifi_task(
    runner: cyw43::Runner<'static, Output<'static>, PioSpi<'static, PIO0, 0, DMA_CH0>>,
) -> ! {
    runner.run().await
}

#[embassy_executor::task]
async fn net_task(mut runner: embassy_net::Runner<'static, cyw43::NetDriver<'static>>) -> ! {
    runner.run().await
}

#[embassy_executor::task]
pub async fn networking(spawner: Spawner, r: WifiResources) -> ! {
    info!("Networking task started");

    // To make flashing faster for development, you may want to flash the firmwares independently
    // at hardcoded addresses, instead of baking them into the program with `include_bytes!`:
    //     probe-rs-cli download 43439A0.bin --format bin --chip RP2040 --base-address 0x10100000
    //     probe-rs-cli download 43439A0_clm.bin --format bin --chip RP2040 --base-address 0x10140000
    // let fw = unsafe { core::slice::from_raw_parts(0x10100000 as *const u8, 224190) };
    // let clm = unsafe { core::slice::from_raw_parts(0x10140000 as *const u8, 4752) };
    let fw = include_bytes!("../../bin/cyw43-firmware/43439A0.bin");
    let clm = include_bytes!("../../bin/cyw43-firmware/43439A0_clm.bin");

    // Configure PIO and CYW43
    let pwr = Output::new(r.pwr_pin, Level::Low);
    let cs = Output::new(r.cs_pin, Level::High);
    let mut pio = Pio::new(r.pio_sm, Irqs);
    let spi = PioSpi::new(
        &mut pio.common,
        pio.sm0,
        pio.irq0,
        cs,
        r.dio_pin,
        r.clk_pin,
        r.dma_ch,
    );

    static STATE: StaticCell<cyw43::State> = StaticCell::new();
    let state = STATE.init(cyw43::State::new());
    let (net_device, mut control, runner) = cyw43::new(state, pwr, spi, fw).await;

    unwrap!(spawner.spawn(wifi_task(runner)));

    info!("Init control");
    control.init(clm).await;
    control
        .set_power_management(cyw43::PowerManagementMode::PowerSave)
        .await;

    const WIFI_NETWORK: &str = env!("WIFI_NETWORK");
    const WIFI_PASSWORD: &str = env!("WIFI_PASSWORD");
    const CLIENT_NAME: &str = "picow";

    let mut dhcp_config = DhcpConfig::default();
    dhcp_config.hostname = Some(heapless::String::from_str(CLIENT_NAME).unwrap());
    let net_config = NetConfig::dhcpv4(dhcp_config);

    // Generate random seed
    let seed = RoscRng.next_u64();

    // Init network stack
    static RESOURCES: StaticCell<StackResources<5>> = StaticCell::new();
    let (stack, runner) = embassy_net::new(net_device, net_config, RESOURCES.init(StackResources::new()), seed);

    unwrap!(spawner.spawn(net_task(runner)));
    
    loop {
        match control
            .join_wpa2(WIFI_NETWORK, WIFI_PASSWORD)
            .await
        {
            Ok(_) => break,
            Err(err) => {
                info!("join failed with status={}", err.status);
            }
        }
    }

    // Wait for DHCP, not necessary when using static IP
    info!("waiting for DHCP...");
    while !stack.is_config_up() {
        Timer::after_millis(100).await;
    }
    info!("DHCP is now up!");

    info!("waiting for link up...");
    while !stack.is_link_up() {
        Timer::after_millis(500).await;
    }
    info!("Link is up!");

    info!("waiting for stack to be up...");
    stack.wait_config_up().await;
    info!("Stack is up!");

    // // And now we can use it!

    loop {
        info!("Blink led");
        control.gpio_set(0, false).await;
        Timer::after(Duration::from_secs(3)).await;
        control.gpio_set(0, true).await;
        Timer::after(Duration::from_millis(20)).await;
    }

}
