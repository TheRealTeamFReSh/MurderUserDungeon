use bevy::prelude::*;

use crate::console::event::PrintConsoleEvent;

pub fn game_loop(
    //mut cg_data: ResMut<ConsoleGamesData>,
    mut _console_writer: EventWriter<PrintConsoleEvent>,
) {
    //console_writer.send(PrintConsoleEvent("GameLoop".to_string()));
}