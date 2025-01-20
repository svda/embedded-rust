//! # Task messages of the system
//! This module contains the messages that we want to send between the tasks. We have two types of messages: events and commands.
//! Events are the messages that we want the orchestrator to react to. They contain the data that we need to react to the event.
//! Commands are the messages that we want the orchestrator to send to the other tasks that we want to control. They contain the data that we need to send to the other tasks.
//! The messages are sent through channels and signals. The channels are used for the events and the commands are sent through the signals.

use defmt::Format;
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::channel::Channel;

/// Events that we want to react to together with the data that we need to react to the event.
/// Works in conjunction with the `EVENT_CHANNEL` channel in the orchestrator task.
#[derive(PartialEq, Debug, Format)]
pub enum Events {
    /// The button was pressed, the data is the number of presses
    Btn,
    /// The system must go to standby mode
    Standby,
    /// The system must wake up from standby mode
    WakeUp,
}

/// For the events that we want the orchestrator to react to, all state events are of the type Enum Events.
pub static EVENT_CHANNEL: Channel<CriticalSectionRawMutex, Events, 10> = Channel::new();
