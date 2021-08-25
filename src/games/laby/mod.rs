mod commands;
mod game;
mod data;
mod walls;

use bevy::{ecs::schedule::ShouldRun, prelude::*};

use crate::{games::GameList, states::GameState};

use super::ConsoleGamesData;

pub struct LabyrinthGamePlugin;

impl Plugin for LabyrinthGamePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.insert_resource(data::LabyrinthData::default());
        app.add_system_set(
            SystemSet::on_update(GameState::ConsoleOpenedState)
                .with_run_criteria(should_run.system())
                .with_system(game::game_loop.system())
                .with_system(commands::commands_handler.system()).before("send_console_input"),
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