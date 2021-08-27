use bevy::prelude::*;

use crate::{
    console::{
        event::{EnteredConsoleCommandEvent, PrintConsoleEvent},
        ConsoleData,
    },
    games::{ConsoleGamesData, GameList},
};

use super::game::{self, TicTacToeData};

pub fn commands_handler(
    mut cmd_reader: EventReader<EnteredConsoleCommandEvent>,
    mut console_writer: EventWriter<PrintConsoleEvent>,
    mut cg_data: ResMut<ConsoleGamesData>,
    mut data: ResMut<ConsoleData>,
    mut ttt_data: ResMut<TicTacToeData>,
) {
    for EnteredConsoleCommandEvent(cmd) in cmd_reader.iter() {
        // Don't do anything if the string is empty
        if cmd.is_empty() {
            return;
        }

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
                ttt_data.reset();
                console_writer.send(PrintConsoleEvent("Quitting TicTacToe...".to_string()));
                cg_data.loaded_game = GameList::None;
                cg_data.ragequit_count += 1;
            }
            "tutorial" => console_writer.send(PrintConsoleEvent(game::display_tutorial())),

            "place" => {
                if args.len() == 1 {
                    console_writer.send(PrintConsoleEvent(
                        "No position provided\nUsage: place <pos>".to_string(),
                    ));
                    return;
                }

                game::play_position(args[1], &mut ttt_data, &mut console_writer);
            }

            _ => {
                console_writer.send(PrintConsoleEvent(format!(
                    "I didn't understand the command: \"{}\"",
                    args[0]
                )));
            }
        }
    }
}

fn display_help() -> String {
    let mut res = String::from("\nSHOWING 'TicTacToe' COMMANDS\n");

    let underline = "==========================\n\n";
    res.push_str(underline);

    res.push_str("- help : Displays this message\n");
    res.push_str("- clear : Clears commands on the screen\n");
    res.push_str("- tutorial : Show the tutorial for this game\n");
    res.push_str("- place <pos> : Place a pawn at the position <pos>\n");
    res.push_str("- ragequit : Leaves the game (you will lose your progress)\n");

    res
}
