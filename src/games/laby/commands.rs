use bevy::prelude::*;
use rand::Rng;

use crate::{
    console::{
        event::{EnteredConsoleCommandEvent, PrintConsoleEvent},
        ConsoleData,
    },
    games::ConsoleGamesData,
    npcs::NPCsResource,
    vulnerability::VulnerabilityResource,
};

use super::{
    data::{
        GameState, LabyrinthData, LabyrinthResourceFile, Movement, PlayerActions, PlayerStats,
        RoomType,
    },
    enemies::EnemyType,
    game::new_turn,
};

pub fn commands_handler(
    mut cmd_reader: EventReader<EnteredConsoleCommandEvent>,
    mut console_writer: EventWriter<PrintConsoleEvent>,
    mut cg_data: ResMut<ConsoleGamesData>,
    mut data: ResMut<ConsoleData>,
    mut laby_data: ResMut<LabyrinthData>,
    laby_res: Res<LabyrinthResourceFile>,
    mut vuln_res: ResMut<VulnerabilityResource>,
    mut player: ResMut<PlayerStats>,
    npc_res: Res<NPCsResource>,
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
            "help" => {
                if args.len() == 1 {
                    console_writer.send(PrintConsoleEvent(display_help(1)));
                } else {
                    console_writer.send(PrintConsoleEvent(display_help(args[1].parse().unwrap())))
                }
            }
            "ragequit" => {
                console_writer.send(PrintConsoleEvent("Quitting Labyrinth...".to_string()));
                laby_data.reset();
                player.reset();
                cg_data.ragequit(&mut vuln_res);
            }
            "tutorial" => {
                laby_data.game_state = GameState::Tutorial;
                laby_data.has_shown_turn_infos = false;
            }
            "infos" => {
                laby_data.has_shown_turn_infos = false;
            }
            "continue" => {
                if laby_data.game_state == GameState::Tutorial {
                    laby_data.has_shown_turn_infos = false;
                    laby_data.wait_for_continue = false;

                    if laby_data.tutorial_page + 1 == laby_res.tutorial.len() {
                        laby_data.game_state = GameState::Exploring;
                        new_turn(&mut laby_data, &laby_res, &mut player, &npc_res);
                    } else {
                        laby_data.tutorial_page += 1;
                    }
                } else {
                    console_writer.send(PrintConsoleEvent(
                        "There is nothing to continue...".to_string(),
                    ));
                }
            }
            "skip" => {
                if laby_data.game_state == GameState::Exploring
                    && ((laby_data.room_type == RoomType::Enemy
                        && laby_data.enemy.kind != EnemyType::Boss)
                        || laby_data.room_type == RoomType::Item
                        || laby_data.room_type == RoomType::Npc)
                {
                    console_writer.send(PrintConsoleEvent("Skipping room...".to_string()));
                    laby_data.status_message = "Skipping room...".to_string();
                    new_turn(&mut laby_data, &laby_res, &mut player, &npc_res);
                    laby_data.has_shown_turn_infos = false;
                    laby_data.wait_for_continue = false;
                } else {
                    console_writer
                        .send(PrintConsoleEvent("You can't skip this room...".to_string()));
                }
            }
            "go" => {
                if args.len() == 1 {
                    console_writer.send(PrintConsoleEvent(
                        "You specified no direction...".to_string(),
                    ));
                    console_writer.send(PrintConsoleEvent(
                        "Usage: go <direction>, valid: (FORWARD, LEFT, RIGHT)".to_string(),
                    ));
                    return;
                }

                if let Some(movement) = Movement::from_string(args[1]) {
                    if laby_data.next_directions.can_go_direction(movement) {
                        new_turn(&mut laby_data, &laby_res, &mut player, &npc_res);
                        laby_data.has_shown_turn_infos = false;
                        laby_data.wait_for_continue = false;
                    } else {
                        console_writer.send(PrintConsoleEvent(
                            "There is no path in this direction...".to_string(),
                        ));
                    }
                    return;
                } else {
                    console_writer.send(PrintConsoleEvent(
                        "Please enter a valid direction...".to_string(),
                    ));
                    console_writer.send(PrintConsoleEvent(
                        "Usage: go <direction>, valid: (FORWARD, LEFT, RIGHT)".to_string(),
                    ));
                }
            }
            "attack" => {
                if laby_data.room_type == RoomType::Enemy {
                    // player phase
                    let damages = {
                        if player.last_action == PlayerActions::Prepare {
                            player.damages * 2.5
                        } else {
                            player.damages
                        }
                    };
                    let atk_msg = format!("Attacking the enemy for {} damage", damages);
                    console_writer.send(PrintConsoleEvent(atk_msg.clone()));
                    laby_data.status_message = atk_msg.clone();
                    laby_data.enemy.health -= damages;

                    ai_turn(&mut player, &mut laby_data);
                } else {
                    console_writer.send(PrintConsoleEvent(
                        "You punch... uh... the wall!".to_string(),
                    ));
                    console_writer.send(PrintConsoleEvent(
                        "In fustration, you see there is nothing else to punch here!".to_string(),
                    ));
                }
            }
            "prepare" => {
                if laby_data.room_type == RoomType::Enemy {
                    let msg = String::from("You prepare yourself for your next attack (x2.5)");
                    console_writer.send(PrintConsoleEvent(msg.clone()));
                    laby_data.status_message = msg.clone();
                    player.action = PlayerActions::Prepare;

                    ai_turn(&mut player, &mut laby_data);
                } else {
                    console_writer.send(PrintConsoleEvent(
                        "There is no need to prepare an attack here...".to_string(),
                    ));
                }
            }
            "protect" => {
                if laby_data.room_type == RoomType::Enemy {
                    let msg = String::from("You put yourself in a protection position (x0.5)");
                    console_writer.send(PrintConsoleEvent(msg.clone()));
                    laby_data.status_message = msg.clone();
                    player.action = PlayerActions::Protect;

                    ai_turn(&mut player, &mut laby_data);
                } else {
                    console_writer.send(PrintConsoleEvent(
                        "You protect yourself from the strange liquid coming from the ceiling on your head...".to_string(),
                    ));
                }
            }
            "talk" => {
                if laby_data.room_type == RoomType::Npc {
                    let will_give_boon = rand::thread_rng().gen_ratio(2, 10);

                    let msg: String;
                    if will_give_boon {
                        msg = "The NPC was happy you talked with him and wants to give you something".to_string();
                        player.health += 1.0;
                        player.damages += 1.0;
                    } else {
                        msg = "I think you just made a friend! Pretty nice huh?".to_string();
                    }

                    laby_data.has_shown_turn_infos = false;
                    laby_data.wait_for_continue = false;

                    console_writer.send(PrintConsoleEvent(msg.clone()));
                    laby_data.status_message = msg.clone();
                    new_turn(&mut laby_data, &laby_res, &mut player, &npc_res);
                } else {
                    console_writer.send(PrintConsoleEvent("You try to talk to yourself and gained nothing but loneliness..".to_string()));
                }
            }
            "insult" => {
                if laby_data.room_type == RoomType::Npc {
                    let will_give_boon = rand::thread_rng().gen_ratio(9, 10);

                    let msg: String;
                    if will_give_boon {
                        msg = "You become stronger by insulting this NPC. But you've made a new enemy...".to_string();
                        player.health += 1.0;
                        player.damages += 1.0;
                        // TODO: add the enemy to the vuln vec
                    } else {
                        msg = "The NPC just doesn't care, he just goes away".to_string();
                    }

                    laby_data.has_shown_turn_infos = false;
                    laby_data.wait_for_continue = false;

                    console_writer.send(PrintConsoleEvent(msg.clone()));
                    laby_data.status_message = msg.clone();
                    new_turn(&mut laby_data, &laby_res, &mut player, &npc_res);
                } else {
                    console_writer.send(PrintConsoleEvent("You insult the void and reminds you of your loneliness..".to_string()));
                }
            }
            "loot" => {
                if laby_data.room_type == RoomType::Item {
                    let will_give_boon = rand::thread_rng().gen_ratio(7, 10);

                    let msg: String;
                    if will_give_boon {
                        msg = "You managed to find a good item and gained damages".to_string();
                        player.damages += 0.5;
                    } else {
                        msg = "Ah crap its too rusty... maybe next time?".to_string();
                    }

                    laby_data.has_shown_turn_infos = false;
                    laby_data.wait_for_continue = false;

                    console_writer.send(PrintConsoleEvent(msg.clone()));
                    laby_data.status_message = msg.clone();
                    new_turn(&mut laby_data, &laby_res, &mut player, &npc_res);
                } else {
                    console_writer.send(PrintConsoleEvent("You try to loot this room..\n There is nothing but rocks (sry)...".to_string()));
                }
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

fn ai_turn(player: &mut ResMut<PlayerStats>, laby_data: &mut ResMut<LabyrinthData>) {
    laby_data.has_shown_turn_infos = false;
    laby_data.wait_for_continue = false;
    let damages = {
        if player.last_action == PlayerActions::Protect {
            laby_data.enemy.damages * 0.5
        } else {
            laby_data.enemy.damages
        }
    };
    laby_data
        .status_message
        .push_str(&format!("\nThe enemy attacks you for {} HP.", damages,));
    player.health -= damages;
    player.health = player.health.max(0.0);
}

fn display_help(page_number: usize) -> String {
    let mut res = String::from("\nSHOWING 'Labyrinth' COMMANDS\n");

    let underline = "============================\n\n";
    res.push_str(underline);

    if page_number == 1 {
        res.push_str("- help: Displays this message\n");
        res.push_str("- clear: Clears commands on the screen\n");
        res.push_str("- tutorial: Show the tutorial for this game\n");
        res.push_str("- go <direction>: Move the player to the next direction\n");
        res.push_str("- ragequit: Leaves the game (you will lose your progress)\n");
        res.push_str("- infos: Display informations about the place you stand\n");
        res.push_str("- skip: skip this room to go to the next (if you can)\n");
    } else {
        res.push_str("- talk: talks to an npc to maybe receive a boon\n");
        res.push_str("- insult: insults the npc to become stronger\n");
        res.push_str("- loot: loots the item (when you find one)\n");
        res.push_str("- continue: to continue a story/speech\n");
        res.push_str("- attack: attacks the monster / NPC\n");
        res.push_str("- prepare: prepares the attack for x2.5 damages\n");
        res.push_str("- protect: a protection position to take x0.5 damages\n");
    }

    res.push_str(&format!("\n============({}/2)===========\n", page_number));

    res
}
