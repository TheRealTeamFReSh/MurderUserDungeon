mod commands;
mod game;
mod data;
mod art;
mod enemies;
mod items;

use bevy::{ecs::schedule::ShouldRun, prelude::*};
use ron::de::from_bytes;

use crate::{games::GameList, states::GameState};

use super::ConsoleGamesData;

pub struct LabyrinthGamePlugin;

impl Plugin for LabyrinthGamePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.insert_resource(data::LabyrinthData::default());
        app.insert_resource(
            from_bytes::<data::LabyrinthResourceFile>(include_bytes!(
                "../../../data/labyrinth_data.ron"
            ))
            .unwrap(),
        );
        app.add_system_set(
            SystemSet::on_update(GameState::ConsoleOpenedState)
                .with_run_criteria(should_run.system())
                .with_system(
                    game::game_loop.system()
                        .label("laby_game_loop")
                        .before("laby_cmd_handler")
                )
                .with_system(
                    commands::commands_handler.system()
                        .label("laby_cmd_handler")
                        .before("send_console_input")
                ),
        );
    }
}

pub fn start_game(
    cg_data: &mut ResMut<ConsoleGamesData>,
) {
    cg_data.loaded_game = GameList::Labyrinth;
    info!("Starting labyrinth game");
}

pub fn should_run(
    cg_data: Res<ConsoleGamesData>,
) -> ShouldRun
{
    if cg_data.loaded_game == GameList::Labyrinth {
        ShouldRun::Yes
    } else {
        ShouldRun::No
    }
}