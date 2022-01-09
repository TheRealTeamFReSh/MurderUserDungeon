mod art;
mod commands;
mod data;
mod enemies;
mod game;
mod items;
mod utils;

use bevy::{ecs::schedule::ShouldRun, prelude::*};
use ron::de::from_bytes;

use crate::{games::GameList, states::GameState};

use super::ConsoleGamesData;

pub struct LabyrinthGamePlugin;

impl Plugin for LabyrinthGamePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.insert_resource(data::LabyrinthData::default());
        app.insert_resource(data::PlayerStats::default());
        app.insert_resource(
            from_bytes::<data::LabyrinthResourceFile>(include_bytes!(
                "../../../data/labyrinth_data.ron"
            ))
            .unwrap(),
        );
        app.add_system_set(
            SystemSet::on_update(GameState::ConsoleOpenedState)
                .with_run_criteria(should_run)
                .with_system(
                    game::game_loop
                        
                        .label("laby_game_loop")
                        .before("laby_cmd_handler"),
                )
                .with_system(
                    commands::commands_handler
                        
                        .label("laby_cmd_handler")
                        .before("send_console_input"),
                ),
        );
    }
}

pub fn start_game(cg_data: &mut ResMut<ConsoleGamesData>) {
    cg_data.loaded_game = GameList::Labyrinth;
    #[cfg(debug_assertions)]
    info!("Starting labyrinth game");
}

pub fn should_run(cg_data: Res<ConsoleGamesData>) -> ShouldRun {
    if cg_data.loaded_game == GameList::Labyrinth {
        ShouldRun::Yes
    } else {
        ShouldRun::No
    }
}
