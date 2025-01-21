use core::fmt;

use defmt::*;
use embassy_executor::Spawner;
use embassy_time::Timer;
use embassy_rp::i2c::{Config, I2c};
use ens160::{AirQualityIndex, Ens160};

use crate::system::resources::{AirQualityResources, Irqs};

fn trimhex(s: &str) -> &str {
    s.strip_prefix("0x").unwrap_or(s.strip_prefix("0X").unwrap_or(s))
}

#[embassy_executor::task]
pub async fn air_quality(_spawner: Spawner, r: AirQualityResources) -> ! {
    info!("Air quality task started");

    let mut config = Config::default();
    config.frequency = 400_000;

    let i2c_bus = I2c::new_async(
        r.i2c,
        r.scl,
        r.sda,
       Irqs,
       config,
    );

    let mut ens160 = Ens160::new(i2c_bus, 0x53);

    ens160.reset().unwrap();
    Timer::after_millis(1000).await;
    ens160.operational().unwrap();

    loop {
        if let Ok(status) = ens160.status() {
            if status.data_is_ready() {
                let temp_hum = ens160.temp_and_hum().unwrap();
                info!("Temperature: {}", temp_hum.0/100);
                info!("Humidity: {}", temp_hum.1/100);
                let tvoc = ens160.tvoc().unwrap();
                info!("TVOC: {}", tvoc);
                let eco2 = ens160.eco2().unwrap().to_le();
                info!("CO2: {}", eco2);

                // from eCO2
                //let air_quality_index = AirQualityIndex::try_from(eco2).unwrap();
                // directly
                //let air_quality_index = ens160.air_quality_index().unwrap();
                //info!("Air quality: {}", air_quality_index);
            }
        }
        Timer::after_millis(5000).await;
    }
    
}
