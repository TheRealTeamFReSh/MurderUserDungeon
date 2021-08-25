use bevy::prelude::*;
use rand::Rng;

use crate::{console::{ConsoleData, event::PrintConsoleEvent}, games::laby::data::Directions};

use super::{data::{GameState, LabyrinthData, LabyrinthResourceFile, RoomType}, enemies::EnemyType, items::ItemType};

pub fn game_loop(
    mut laby_data: ResMut<LabyrinthData>,
    laby_res: Res<LabyrinthResourceFile>,
    mut console_writer: EventWriter<PrintConsoleEvent>,
    mut console_data: ResMut<ConsoleData>,
) {
    if laby_data.has_shown_turn_infos || laby_data.wait_for_continue { return; }

    // first we clear the screen
    console_data.messages.clear();

    match laby_data.game_state {
        // if we're going through the tutorial
        GameState::Tutorial => {
            console_writer.send(PrintConsoleEvent(display_tutorial(&laby_res)));
            laby_data.wait_for_continue = true;
        },

        // if it's just about exploring
        GameState::Exploring => {
            match laby_data.room_type {
                RoomType::Corridor => console_writer.send(PrintConsoleEvent(turn_display(&laby_data))),

                RoomType::Enemy => console_writer.send(PrintConsoleEvent(enemy_display(&laby_data, &laby_res))),

                RoomType::Item => console_writer.send(PrintConsoleEvent(item_display(&laby_data, &laby_res))),
            }
        },  
    };

    // in order to not see this message again
    laby_data.has_shown_turn_infos = true;
}

fn display_tutorial(
    laby_res: &Res<LabyrinthResourceFile>,
) -> String {
    let mut res = String::from("------------------==[Labyrinth]==-----------------\n\n");

    res.push_str(&laby_res.tutorial);
    res.push_str("\n\n\n");

    res.push_str("Don't forget to type 'help' for the list of commands.\n");
    res.push_str("To show this tutorial type 'tutorial', to show the\n");
    res.push_str("informations about the current room type 'infos'\n");
    res.push_str("To navigate in the labyrinth type 'go <dir>'\n\n");
    res.push_str("Type 'continue' in order to start the adventure...\n");

    res
}

fn turn_display(
    laby_data: &ResMut<LabyrinthData>,
) -> String {
    // Map display
    let mut res = String::from("----------------------[View]----------------------\n");
    res.push_str(laby_data.next_directions.get_ascii_art());
    res.push('\n');
    res.push_str(&format!("Number of steps since the beginning: {}\n", laby_data.steps_number));
    res.push_str(&format!("Available movements: [{}]\n\n", laby_data.next_directions.to_display()));

    // Description
    res.push_str("-------------------[Description]------------------\n");
    res.push_str(&format!("{}\n\n", laby_data.description));

    res
}

fn enemy_display(
    laby_data: &ResMut<LabyrinthData>,
    laby_res: &Res<LabyrinthResourceFile>,
) -> String {
    let mut res = String::from("----------------------[View]----------------------\n");
    res.push_str(laby_data.enemy_type.get_ascii_art());
    res.push('\n');

    // Description
    res.push_str("-------------------[Description]------------------\n");
    res.push_str(&format!("{}\n\n", laby_res.enemy_desc.get(laby_data.enemy_type.to_display_str()).unwrap()));

    res
}

fn item_display(
    laby_data: &ResMut<LabyrinthData>,
    _laby_res: &Res<LabyrinthResourceFile>,
) -> String {
    let mut res = String::from("----------------------[View]----------------------\n");
    res.push_str(laby_data.item_type.get_ascii_art());
    res.push('\n');

    // Description
    res.push_str("-------------------[Description]------------------\n");
    //res.push_str(&format!("{}\n\n", laby_res.enemy_desc.get(laby_data.enemy_type.to_display_str()).unwrap()));

    res
}

pub fn new_turn(
    laby_data: &mut ResMut<LabyrinthData>,
    laby_res: &Res<LabyrinthResourceFile>,
) {
    laby_data.steps_number += 1;

    let is_next_enemy = rand::thread_rng().gen_ratio(1, 3);
    let is_next_item = rand::thread_rng().gen_ratio(1, 3);

    if is_next_enemy {
        laby_data.room_type = RoomType::Enemy;
        laby_data.enemy_type = EnemyType::get_random_enemy();
    } else if is_next_item {
        laby_data.room_type = RoomType::Item;
        laby_data.item_type = ItemType::get_random_item();
    } else {
        laby_data.room_type = RoomType::Corridor;
        laby_data.next_directions = Directions::get_random_direction();

        let index = rand::thread_rng().gen_range(0..laby_res.descriptions.len());
        laby_data.description = laby_res.descriptions.get(index).unwrap().clone();
    }
}