use bevy::prelude::*;

use crate::{console::{ConsoleData, event::{EnteredConsoleCommandEvent, PrintConsoleEvent}}, games::{ConsoleGamesData, GameList}};

pub fn commands_handler(
    mut cmd_reader: EventReader<EnteredConsoleCommandEvent>,
    mut console_writer: EventWriter<PrintConsoleEvent>,
    mut cg_data: ResMut<ConsoleGamesData>,
    mut data: ResMut<ConsoleData>,
) {
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
            "help" => {
                if args.len() == 1 {
                    console_writer.send(PrintConsoleEvent(display_help(1)));
                } else {
                    console_writer.send(PrintConsoleEvent(display_help(args[1].parse().unwrap())))
                }
                
            },
            "ragequit" => {
                console_writer.send(PrintConsoleEvent("Quitting Labyrinth...".to_string()));
                cg_data.loaded_game = GameList::None;
            },
            "tutorial" => console_writer.send(PrintConsoleEvent("Displaying tutorial...".to_string())),

            _ => {
                console_writer.send(PrintConsoleEvent(format!("I didn't understand the command: \"{}\"", args[0])));
            }
        }
    }
}

fn display_help(page_number: usize) -> String {
    let mut res = String::from("\nSHOWING 'Labyrinth' COMMANDS\n");

    let underline  = "==========================\n\n";
    res.push_str(underline);

    if page_number == 1 {
        res.push_str("- help : Displays this message\n");
        res.push_str("- clear : Clears commands on the screen\n");
        res.push_str("- tutorial : Show the tutorial for this game\n");
        res.push_str("- go <direction> : Move the player to the next direction\n");
        res.push_str("- ragequit : Leaves the game (you will lose your progress)\n");
    } else {
        res.push_str("No more commands\n");
    }

    res.push_str(&format!("\n===========({}/2)==========\n", page_number));

    res
}