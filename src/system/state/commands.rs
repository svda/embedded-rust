use defmt::Format;
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::signal::Signal;

/// Commands that we want to send from the orchestrator to the other tasks that we want to control.
/// Works in conjunction with the `COMMAND_CHANNEL` channel in the orchestrator task.
#[derive(PartialEq, Debug, Format)]
pub enum Commands {
    /// Since we will need to update the display often and wizth a lot of data, we will not send the data in the command option
    DisplayUpdate,
}

/// For the update commands that we want the orchestrator to send to the display task. Since we only ever want to display according to the state of
/// the system, we will not send any data in the command option and we can afford to work only with a simple state of "the display needs to be updated".
pub static DISPLAY_SIGNAL: Signal<CriticalSectionRawMutex, Commands> = Signal::new();
