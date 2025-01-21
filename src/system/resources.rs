//! # Resources
//! this module is used to define the resources that will be used in the tasks
//!
//! the resources are defined in the main.rs file, and assigned to the tasks in the main.rs file
use assign_resources::assign_resources;
use embassy_rp::i2c::InterruptHandler as I2cInterruptHandler;
use embassy_rp::peripherals::{I2C0, PIO0};
use embassy_rp::pio::InterruptHandler as PioInterruptHandler;
use embassy_rp::{bind_interrupts, peripherals};

// group the peripherlas into resources, to be used in the tasks
// the resources are assigned to the tasks in main.rs
assign_resources! {
    air_quality: AirQualityResources {
        i2c: I2C0,
        sda: PIN_12,
        scl: PIN_13,
    },
    wifi: WifiResources {
        pwr_pin: PIN_23,
        cs_pin: PIN_25,
        pio_sm: PIO0,
        dio_pin: PIN_24,
        clk_pin: PIN_29,
        dma_ch: DMA_CH0,
    },
}

// bind the interrupts, on a global scope for convenience
bind_interrupts!(pub struct Irqs {
    I2C0_IRQ => I2cInterruptHandler<I2C0>;
    PIO0_IRQ_0 => PioInterruptHandler<PIO0>;
});
