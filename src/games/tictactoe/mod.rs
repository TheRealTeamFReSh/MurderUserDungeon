mod commands;
mod game;

use bevy::{ecs::schedule::ShouldRun, prelude::*};

use crate::{games::GameList, states::GameState};

use super::ConsoleGamesData;

pub struct TicTacToePlugin;

impl Plugin for TicTacToePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.insert_resource(game::TicTacToeData::default());
        app.add_system_set(
            SystemSet::on_update(GameState::ConsoleOpenedState)
                .with_run_criteria(should_run)
                .with_system(game::game_loop)
                .with_system(commands::commands_handler)
                .before("send_console_input"),
        );
    }
}

pub fn start_game(cg_data: &mut ResMut<ConsoleGamesData>) {
    cg_data.loaded_game = GameList::TicTacToe;
    #[cfg(debug_assertions)]
    info!("Starting tictactoe game");
}

pub fn should_run(cg_data: Res<ConsoleGamesData>) -> ShouldRun {
    if cg_data.loaded_game == GameList::TicTacToe {
        ShouldRun::Yes
    } else {
        ShouldRun::No
    }
}
