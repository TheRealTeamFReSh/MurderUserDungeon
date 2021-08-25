use bevy::prelude::*;

use crate::{console::{ConsoleData, event::{EnteredConsoleCommandEvent, PrintConsoleEvent}}, games::{ConsoleGamesData, GameList}};

use super::{data::{GameState, LabyrinthData, LabyrinthResourceFile, Movement}, game::new_turn};

pub fn commands_handler(
    mut cmd_reader: EventReader<EnteredConsoleCommandEvent>,
    mut console_writer: EventWriter<PrintConsoleEvent>,
    mut cg_data: ResMut<ConsoleGamesData>,
    mut data: ResMut<ConsoleData>,
    mut laby_data: ResMut<LabyrinthData>,
    laby_res: Res<LabyrinthResourceFile>,
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
                laby_data.reset();
                cg_data.loaded_game = GameList::None;
            },
            "tutorial" => {
                laby_data.game_state = GameState::Tutorial;
                laby_data.has_shown_turn_infos = false;
            },
            "infos" => {
                laby_data.has_shown_turn_infos = false;
            }
            "continue" => {
                if laby_data.game_state == GameState::Tutorial {
                    laby_data.game_state = GameState::Exploring;
                    new_turn(&mut laby_data, &laby_res);
                    laby_data.has_shown_turn_infos = false;
                    laby_data.wait_for_continue = false;
                } else {
                    console_writer.send(PrintConsoleEvent("There is nothing to continue...".to_string()));
                }
            }
            "go" => {
                if args.len() == 1 {
                    console_writer.send(PrintConsoleEvent("You specified no direction...".to_string()));
                    console_writer.send(PrintConsoleEvent("Usage: go <direction>, valid: (FORWARD, LEFT, RIGHT)".to_string()));
                    return;
                }

                if let Some(movement) = Movement::from_string(args[1]) {
                    if laby_data.next_directions.can_go_direction(movement) {
                        new_turn(&mut laby_data, &laby_res);
                    } else {
                        console_writer.send(PrintConsoleEvent("There is no path in this direction...".to_string()));
                    }
                    return;
                }
            }

            _ => {
                console_writer.send(PrintConsoleEvent(format!("I didn't understand the command: \"{}\"", args[0])));
            }
        }
    }
}

fn display_help(page_number: usize) -> String {
    let mut res = String::from("\nSHOWING 'Labyrinth' COMMANDS\n");

    let underline  = "============================\n\n";
    res.push_str(underline);

    if page_number == 1 {
        res.push_str("- help: Displays this message\n");
        res.push_str("- clear: Clears commands on the screen\n");
        res.push_str("- tutorial: Show the tutorial for this game\n");
        res.push_str("- go <direction>: Move the player to the next direction\n");
        res.push_str("- ragequit: Leaves the game (you will lose your progress)\n");
        res.push_str("- infos: Display informations about the place you stand\n");
    } else {
        res.push_str("- continue: to continue a story/speech\n");
    }

    res.push_str(&format!("\n============({}/2)===========\n", page_number));

    res
}