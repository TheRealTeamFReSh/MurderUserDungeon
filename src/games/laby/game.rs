use core::time;
use std::thread;

use bevy::prelude::*;
use rand::{prelude::SliceRandom, Rng};

use crate::{console::{event::PrintConsoleEvent, ConsoleData}, games::{
        laby::{
            art,
            data::{Directions, PlayerActions},
            utils::{self, display_bar},
        },
        ConsoleGamesData, GameList,
    }, npcs::{NPCData, NPCsResource}, vulnerability::{BoolVulnerabilityType, VulnerabilityResource}};

use super::{
    data::{GameState, LabyrinthData, LabyrinthResourceFile, PlayerStats, RoomType},
    enemies::{Enemy, EnemyType},
    items::ItemType,
};

const NUMBER_OF_TURN_TO_WIN: usize = 30;

pub fn game_loop(
    mut laby_data: ResMut<LabyrinthData>,
    mut laby_res: ResMut<LabyrinthResourceFile>,
    mut console_writer: EventWriter<PrintConsoleEvent>,
    mut console_data: ResMut<ConsoleData>,
    mut player: ResMut<PlayerStats>,
    mut vuln_res: ResMut<VulnerabilityResource>,
    npc_res: Res<NPCsResource>,
    mut cg_data: ResMut<ConsoleGamesData>,
) {
    if laby_data.has_shown_turn_infos || laby_data.wait_for_continue {
        return;
    }

    if player.health <= 0.0 {
        *vuln_res
            .bool_vulnerabilities
            .get_mut(&BoolVulnerabilityType::LabyrinthLosing)
            .unwrap() = true;
        return;
    }

    if player.exp >= 10 {
        player.exp -= 10;
        player.level += 1;
        player.max_health += 2.0;
        player.damages += 1.0;
        player.health += 2.0;
        player.health = player.health.min(player.max_health);

        for enemy in laby_res.enemies.iter_mut() {
            enemy.max_health += 2.0;
            enemy.health = (enemy.health + 2.0).min(enemy.max_health);
            enemy.damages += 0.5;
        }
    }

    // first we clear the screen
    console_data.messages.clear();

    match laby_data.game_state {
        // if we're going through the tutorial
        GameState::Tutorial => {
            console_writer.send(PrintConsoleEvent(display_tutorial(
                &laby_res,
                laby_data.tutorial_page,
            )));
            laby_data.wait_for_continue = true;
        }

        // if it's just about exploring
        GameState::Exploring => {
            match laby_data.room_type {
                RoomType::Corridor => {
                    console_writer.send(PrintConsoleEvent(turn_display(&laby_data)))
                }

                RoomType::Enemy => {
                    if laby_data.enemy.health <= 0.0 {
                        laby_data.enemy.health = laby_data.enemy.health.max(0.0);
                        laby_data.wait_for_continue = false;
                        laby_data.has_shown_turn_infos = false;

                        if laby_data.enemy.kind == EnemyType::Boss {
                            console_writer.send(PrintConsoleEvent(
                                "Congrats! You beat the game!\n".to_string(),
                            ));
                            cg_data.loaded_game = GameList::None;
                            cg_data.has_won_laby = true;
                            laby_data.reset();
                            laby_res.reset();
                            player.reset();
                            return;
                        }

                        laby_data.status_message = format!(
                            "Enemy killed! Congrats!\nYou gained {} Exp",
                            laby_data.enemy.exp
                        );
                        player.exp += laby_data.enemy.exp;
                        player.action = PlayerActions::Attack;

                        new_turn(&mut laby_data, &laby_res, &mut player, &npc_res);
                        return;
                    }

                    console_writer.send(PrintConsoleEvent(enemy_display(&laby_data)));
                }

                RoomType::Item => {
                    console_writer.send(PrintConsoleEvent(item_display(&laby_data)))
                }

                RoomType::Npc => console_writer.send(PrintConsoleEvent(npc_display(&laby_data))),
            };
            console_writer.send(PrintConsoleEvent(player_infos(&player)));

            console_writer.send(PrintConsoleEvent(display_status(&laby_data)));

            player.last_action = player.action;
        }
    };

    // in order to not see this message again
    laby_data.has_shown_turn_infos = true;
    // clear the status message for the next pass
    laby_data.status_message = String::from("");
}

fn display_status(laby_data: &ResMut<LabyrinthData>) -> String {
    let mut res = String::from("");
    res.push_str(&format!("{}\n", laby_data.status_message));

    res
}

fn display_tutorial(laby_res: &ResMut<LabyrinthResourceFile>, page: usize) -> String {
    let mut res = String::from("------------------==[Labyrinth]==-----------------\n\n");

    res.push_str(laby_res.tutorial.get(page).unwrap());
    res.push_str("\n\n\n");

    res.push_str("Don't forget to type 'help' for the list of commands.\n");
    res.push_str("To show this tutorial type 'tutorial', to show the\n");
    res.push_str("informations about the current room type 'infos'\n");
    res.push_str("To navigate in the labyrinth type 'go <dir>'\n\n");
    res.push_str("Type 'continue' in order to start the adventure...\n");

    res
}

fn turn_display(laby_data: &ResMut<LabyrinthData>) -> String {
    // Map display
    let mut res = String::from("----------------------[View]----------------------\n");
    res.push_str(laby_data.next_directions.get_ascii_art());
    res.push('\n');
    res.push_str(&format!(
        "Room number: {}/{}\n",
        laby_data.steps_number,
        NUMBER_OF_TURN_TO_WIN
    ));
    res.push_str(&format!(
        "Available movements: [{}]\n\n",
        laby_data.next_directions.to_display()
    ));

    // Description
    res.push_str("-------------------[Description]------------------\n");
    res.push_str(&format!("{}\n", laby_data.description));

    res
}

fn enemy_display(laby_data: &ResMut<LabyrinthData>) -> String {
    let mut res = String::from("----------------------[View]----------------------\n");
    res.push_str(laby_data.enemy.get_ascii_art());
    res.push('\n');

    res.push_str(&format!(
        "Health: {}\n",
        display_bar(
            20,
            laby_data.enemy.health.into(),
            laby_data.enemy.max_health.into()
        )
    ));
    res.push('\n');

    // Description
    res.push_str("-------------------[Description]------------------\n");
    res.push_str(&format!("{}\n", laby_data.enemy.description));

    res
}

fn item_display(
    laby_data: &ResMut<LabyrinthData>,
) -> String {
    let mut res = String::from("----------------------[View]----------------------\n");
    res.push_str(laby_data.item_type.get_ascii_art());
    res.push('\n');

    // Description
    res.push_str("-------------------[Description]------------------\n");
    res.push_str(
        "
This item has currently no description
However you can type 'loot' to get its content
or you can type 'skip' to go to the next room.\n\n",
    );

    res
}

fn npc_display(laby_data: &ResMut<LabyrinthData>) -> String {
    let mut res = String::from("----------------------[View]----------------------\n");
    res.push_str(art::KNIGHT);
    res.push('\n');

    res.push_str(&format!("Username: {}\n", laby_data.npc.username));

    // Description
    res.push_str("---------------------[Talking]--------------------\n");
    res.push_str(
        "
For now all npcs are mute (how so?)
But he is still nice and if you talk with him/her
you may get a small boon from it *wink wink*\n\n",
    );

    res.push_str("Type 'talk' to speak with him/her 'insult' to .. insult the npc\nor 'skip' to go to the next room\n");

    res
}

fn player_infos(player: &ResMut<PlayerStats>) -> String {
    let mut res = String::from("------------------[Player Stats]------------------\n\n");

    res.push_str(&format!(
        "Level: {} | Exp: {} | Gold: 0\n",
        player.level, player.exp,
    ));
    res.push_str(&format!(
        "Health: {}\n",
        utils::display_bar(20, player.health as f64, player.max_health as f64)
    ));

    res
}

pub fn new_turn(
    laby_data: &mut ResMut<LabyrinthData>,
    laby_res: &ResMut<LabyrinthResourceFile>,
    player: &mut ResMut<PlayerStats>,
    npc_res: &Res<NPCsResource>,
) {
    // first wait a bit
    thread::sleep(time::Duration::from_secs_f32(rand::thread_rng().gen_range(0.2..=0.7)));

    laby_data.steps_number += 1;

    player.health += 1.0;
    player.health = player.health.min(player.max_health);

    if laby_data.steps_number == NUMBER_OF_TURN_TO_WIN {
        laby_data.room_type = RoomType::Enemy;
        laby_data.enemy = Enemy {
            damages: 2.0,
            description: "
This is the end. He is the devil, the incarnation
of your worst fears. He is also ...xxXDarkKevin420Xxx."
                .to_string(),
            exp: 100,
            health: 25.0,
            max_health: 25.0,
            kind: EnemyType::Boss,
        };
        return;
    }

    // if we were in tutorial we go in corridor
    if laby_data.steps_number <= 1 {
        laby_data.room_type = RoomType::Corridor; 
    } else {
        // 10 rooms =
        // 4 corridor, 1 item, 3 enemy, 2 npc
        let rooms_possibilites = [
            RoomType::Corridor,
            RoomType::Corridor,
            RoomType::Corridor,
            RoomType::Corridor,
            RoomType::Item,
            RoomType::Enemy,
            RoomType::Enemy,
            RoomType::Enemy,
            RoomType::Npc,
            RoomType::Npc,
        ];

        let mut new_room: RoomType;
        loop {
            new_room = *rooms_possibilites.choose(&mut rand::thread_rng()).unwrap();
            
            // if we don't have the same room
            if new_room != laby_data.room_type { break; }
        }
        laby_data.room_type = new_room;
    }

    match laby_data.room_type {
        RoomType::Enemy => {
            laby_data.enemy = Enemy::get_random_enemy(&laby_res.enemies).clone();
        }

        RoomType::Item => {
            laby_data.item_type = ItemType::get_random_item();
        }

        RoomType::Npc => {
            // if there are npcs we haven't seen yet
            if laby_data.seen_npcs.len() < npc_res.npcs.len() {
                let mut npc: NPCData;
                loop {
                    npc = {
                        let index = rand::thread_rng().gen_range(0..npc_res.npcs.values().count());

                        npc_res.npcs.values().nth(index).unwrap().clone()
                    };

                    // if we found someone we didn't found before
                    if !laby_data.seen_npcs.iter().any(|username| username == &npc.username) {
                        break;
                    }
                }
                laby_data.seen_npcs.push(npc.username.clone());
                laby_data.npc = npc;
            }
            // else we show a basic enemy
            else {
                laby_data.room_type = RoomType::Enemy;
                laby_data.enemy = Enemy::get_random_enemy(&laby_res.enemies).clone();
            }
        }

        RoomType::Corridor => {
            laby_data.next_directions = Directions::get_random_direction();

            let index = rand::thread_rng().gen_range(0..laby_res.descriptions.len());
            laby_data.description = laby_res.descriptions.get(index).unwrap().clone();
        }
    }
}
