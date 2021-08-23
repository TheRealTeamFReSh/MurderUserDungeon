use bevy::prelude::*;

use super::ConsoleData;

pub struct PrintConsoleEvent(pub String);
pub struct EnteredConsoleCommandEvent(pub String);

#[allow(dead_code)]
/// Print all sent console messages
pub fn log_console_message_events(
    mut ev_console_message: EventReader<EnteredConsoleCommandEvent>,
) {
    for EnteredConsoleCommandEvent(message) in ev_console_message.iter() {
        info!("MSG: {}", message);
    }
}

pub fn add_message_events_to_console(
    mut data: ResMut<ConsoleData>,
    mut ev_console_message: EventReader<PrintConsoleEvent>,
) {
    for PrintConsoleEvent(message) in ev_console_message.iter() {
        data.messages.push(message.clone());
    }
}