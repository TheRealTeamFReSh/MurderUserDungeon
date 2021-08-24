use bevy::{app::Events, prelude::*};

use crate::{console::{ConsoleData, event::{EnteredConsoleCommandEvent, PrintConsoleEvent}}, games::{ConsoleGamesData, GameList}};

pub fn commands_handler(
    mut cmd_reader: EventReader<EnteredConsoleCommandEvent>,
    mut console_writer: EventWriter<PrintConsoleEvent>,
    mut cg_data: ResMut<ConsoleGamesData>,
    mut data: ResMut<ConsoleData>,
) {
    if cg_data.loaded_game == GameList::None { return ; }

    let mut events = Events::<EnteredConsoleCommandEvent>::default();

    for EnteredConsoleCommandEvent(cmd) in cmd_reader.iter() {
        // Don't do anything if the string is empty
        if cmd.is_empty() { return ; }

        let args: Vec<&str> = cmd.trim().split(' ').collect();

        if args[0] != "clear" {
            // first send what the user typed
            let mut user_input = String::from("> ");
            user_input.push_str(cmd.clone().trim());
            console_writer.send(PrintConsoleEvent(user_input));
        }

        match args[0] {
            "clear" => data.messages.clear(),
            "help" => console_writer.send(PrintConsoleEvent(display_help())),
            "ragequit" => {
                console_writer.send(PrintConsoleEvent("Quitting TicTacToe...".to_string()));
                cg_data.loaded_game = GameList::None;
            },

            _ => {
                console_writer.send(PrintConsoleEvent(format!("I didn't understand the command: \"{}\"", args[0])));
            }
        }
    }
    
    // consuming events
    events.drain().count();
}

fn display_help() -> String {
    let mut res = String::from("\nSHOWING 'TicTacToe' COMMANDS\n");

    let underline  = "==========================\n\n";
    res.push_str(underline);

    res.push_str("- help : Displays this message\n");
    res.push_str("- clear : Clears commands on the screen\n");
    res.push_str("- ragequit : Leaves the game (you will lose your progress)\n");

    res
}