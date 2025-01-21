#![no_std]
#![no_main]
#![allow(async_fn_in_trait)]

use defmt::*;
use embassy_executor::{Executor, InterruptExecutor, Spawner};
use embassy_rp::interrupt;
use embassy_rp::interrupt::{InterruptExt, Priority};
use static_cell::StaticCell;
use {defmt_rtt as _, panic_probe as _};

use crate::system::resources::*;
use crate::tasks::air_quality::air_quality;
use crate::tasks::networking::networking;
use crate::tasks::orchestrator::orchestrator;

// import modules (submodule of src)
mod system;
mod tasks;

// after observing somewhat jumpy behavior of the neopixel task, I decided to set the scheduler and orhestrator to high priority
// hight priority runs on interrupt
static EXECUTOR_HIGH: InterruptExecutor = InterruptExecutor::new();
// low priority runs in thread-mode
static EXECUTOR_LOW: StaticCell<Executor> = StaticCell::new();

#[interrupt]
unsafe fn SWI_IRQ_1() {
    EXECUTOR_HIGH.on_interrupt()
}

// #[cortex_m_rt::pre_init]
// unsafe fn before_main() {
//     // Soft-reset doesn't clear spinlocks. Clear the one used by critical-section
//     // before we hit main to avoid deadlocks when using a debugger
//     embassy_rp::pac::SIO.spinlock(31).write_value(1);
// }

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    info!("Program start");
    // Initialize the peripherals for the RP2040

    let p = embassy_rp::init(Default::default());
    // and assign the peripherals to the places, where we will use them
    let r = split_resources!(p);

    // configure, which tasks to spawn. For a production build we need all tasks, for troubleshooting we can disable some
    // the tasks are all spawned in main.rs, so we can disable them here
    // clutter in the output aside, the binary size is conveniently reduced by disabling tasks
    let task_config = TaskConfig::new();

    // High-priority executor: SWI_IRQ_1, priority level 2
    interrupt::SWI_IRQ_1.set_priority(Priority::P2);
    let spawner = EXECUTOR_HIGH.start(interrupt::SWI_IRQ_1);

    // Orchestrate
    // there is no main loop, the tasks are spawned and run in parallel
    // orchestrating the tasks is done here:
    if task_config.orchestrator {
        spawner.spawn(orchestrator()).unwrap();
    }

    let executor = EXECUTOR_LOW.init(Executor::new());
    executor.run(|spawner| {
        if task_config.air_quality {
            spawner
                .spawn(air_quality(spawner, r.air_quality))
                .unwrap();
        }
        if task_config.networking {
            spawner
                .spawn(networking(spawner, r.wifi))
                .unwrap();
        }
    });
}

/// This struct is used to configure which tasks are enabled
/// This is useful for troubleshooting, as we can disable tasks to reduce the binary size
/// and clutter in the output.
/// Also, we can disable tasks that are not needed for the current development stage and also test tasks in isolation.
/// For a production build we will need all tasks enabled
pub struct TaskConfig {
    pub air_quality: bool,
    pub networking: bool,
    pub orchestrator: bool,
}

impl Default for TaskConfig {
    fn default() -> Self {
        TaskConfig {
            air_quality: true,
            networking: true,
            orchestrator: true,
        }
    }
}

impl TaskConfig {
    pub fn new() -> Self {
        TaskConfig::default()
    }
}
