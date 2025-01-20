use defmt::*;

use crate::system::state::commands::*;
use crate::system::state::events::*;
use crate::system::state::manager::*;

/// This task is responsible for the state transitions of the system. It acts as the main task of the system.
/// It receives events from the other tasks and reacts to them by changing the state of the system.
#[embassy_executor::task]
pub async fn orchestrator() {
    info!("Orchestrate task starting");
    // initialize the state manager and put it into the mutex
    {
        let state_manager = StateManager::new();
        *(STATE_MANAGER_MUTEX.lock().await) = Some(state_manager);
    }

    // init the receiver for the event channel, this is the line we are listening on
    let event_receiver = EVENT_CHANNEL.receiver();

    loop {
        // receive the events, halting the task until an event is received
        let event = event_receiver.receive().await;

        '_state_manager_mutex: {
            // Lock the mutex to get a mutable reference to the state manager
            let mut state_manager_guard = STATE_MANAGER_MUTEX.lock().await;
            // Get a mutable reference to the state manager. We can unwrap here because we know that the state manager is initialized
            let state_manager = state_manager_guard.as_mut().unwrap();

            // react to the events
            match event {
                Events::Btn => {
                    state_manager.handle_button_press().await;
                    DISPLAY_SIGNAL.signal(Commands::DisplayUpdate);
                }
                Events::Standby => {}
                Events::WakeUp => {}
            }
        }
    }
}
