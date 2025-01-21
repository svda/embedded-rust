use core::fmt;

use defmt::*;
use embassy_executor::Spawner;
use embassy_time::Timer;
use embassy_rp::i2c::{Config, I2c};
use ens160_aq::data::{AirQualityIndex, Measurements};
use ens160_aq::Ens160;
use heapless::String;

use crate::system::resources::{AirQualityResources, Irqs};

#[embassy_executor::task]
pub async fn air_quality(spawner: Spawner, r: AirQualityResources) -> ! {
    info!("Air quality task started");

    let mut config = Config::default();
    config.frequency = 400_000;

    let i2c0 = I2c::new_async(
        r.i2c,
        r.scl,
        r.sda,
       Irqs,
       config,
    );

    let mut ens160 = Ens160::new_secondary_address(i2c0, embassy_time::Delay);

    ens160.initialize().await.unwrap();

    loop {
        if let Ok(status) = ens160.get_status().await {
            if status.new_data_ready() {  // read all measurements
                let measurements: Measurements = ens160.get_measurements().await.unwrap();

                info!("CO2: {}", measurements.co2eq_ppm.get_value());
                info!("TVOC: {}", measurements.tvoc_ppb);
                // info!("Quality: {}", measurements.air_quality_index);
                info!("Ethanol: {}", measurements.etoh);
                info!("RR: {}", measurements.raw_resistance);
            }
            // if status.new_group_data_ready() {  // useful to see raw data values
            //     let group_data: [u8; 8] = ens160.get_group_data().await.unwrap();
            //     info!(
            //         "group data = {:#04x} {:#04x} {:#04x} {:#04x} {:#04x} {:#04x} {:#04x} {:#04x}",
            //         group_data[0].to,
            //         group_data[1],
            //         group_data[2],
            //         group_data[3],
            //         group_data[4],
            //         group_data[5],
            //         group_data[6],
            //         group_data[7]
            //     );
            // }
            else {
                info!("no new data ready");
            }  
        }

        Timer::after_millis(5000).await;
    }
}
