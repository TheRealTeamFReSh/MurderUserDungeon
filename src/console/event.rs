use bevy::prelude::*;

use super::ConsoleData;

pub struct PrintConsoleEvent(pub String);
pub struct EnteredConsoleCommandEvent(pub String);

pub fn add_message_events_to_console(
    mut data: ResMut<ConsoleData>,
    mut ev_console_message: EventReader<PrintConsoleEvent>,
) {
    for PrintConsoleEvent(message) in ev_console_message.iter() {
        data.messages.push(message.clone());
    }
}
