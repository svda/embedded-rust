use defmt::*;
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::mutex::Mutex;

use crate::system::state::events::{Events, EVENT_CHANNEL};

/// Type alias for the system state manager protected by a mutex.
///
/// This type alias defines a `Mutex` that uses a `CriticalSectionRawMutex` for synchronization.
/// The state is wrapped in an `Option` to allow for the possibility of the state being uninitialized.
/// This ensures that tasks can safely access and update the state across different executors (e.g., different cores).
type StateManagerType = Mutex<CriticalSectionRawMutex, Option<StateManager>>;

/// Global instance of the system state manager protected by a mutex.
///
/// This static variable holds the system state manager, which is protected by a `Mutex` to ensure
/// that only one task can access the state at a time. The mutex uses a `CriticalSectionRawMutex`
/// for synchronization, allowing safe access across different tasks and executors.
///
/// The state is initially set to `None`, indicating that it has not been initialized yet.
/// Tasks attempting to access the state before initialization will need to handle the `None` case.
pub static STATE_MANAGER_MUTEX: StateManagerType = Mutex::new(None);

/// All the states of the system are kept in this struct.
#[derive(PartialEq, Debug, Format, Clone)]
pub struct StateManager {
    /// The operation mode of the system
    pub operation_mode: OperationMode,
}

/// State transitions
impl StateManager {
    /// Create a new StateManager.             
    /// We will get the actual data pretty early in the system startup, so we can set all this to inits here
    pub fn new() -> Self {
        let manager = StateManager {
            operation_mode: OperationMode::Normal,
        };
        manager
    }

    pub fn set_normal_mode(&mut self) {
        self.operation_mode = OperationMode::Normal;
    }

    pub async fn set_standby_mode(&mut self) {
        let sender = EVENT_CHANNEL.sender();
        self.operation_mode = OperationMode::Standby;
        sender.send(Events::Standby).await;
    }

    pub async fn wake_up(&mut self) {
        let sender = EVENT_CHANNEL.sender();
        self.set_normal_mode();
        sender.send(Events::WakeUp).await;
    }
}

/// User Input Handling
impl StateManager {
    /// Handle state changes when the green button is pressed
    pub async fn handle_button_press(&mut self) {
        match self.operation_mode {
            OperationMode::Normal => {
                self.set_standby_mode().await;
            }
            OperationMode::Standby => {
                self.wake_up().await;
            }
        }
    }
}

/// The operation mode of the system
#[derive(PartialEq, Debug, Format, Clone)]
pub enum OperationMode {
    /// The regular operation mode.
    ///
    /// Displays the time, the alarm status, etc. Showing the analog clock on the neopixel
    /// ring, if the alarm is active.
    Normal,
    /// The system is in standby mode, the display is off, the neopixel ring is off, the system is in a low power state.
    Standby,
}
