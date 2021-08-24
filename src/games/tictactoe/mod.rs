mod commands;
mod game;

use bevy::prelude::*;

use crate::{console::event::PrintConsoleEvent, games::GameList, states::GameState};

use super::ConsoleGamesData;

pub struct TicTacToePlugin;

impl Plugin for TicTacToePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.insert_resource(TicTacToeData {
            has_seen_tutorial: false,
        });
        app.add_system_set(
            SystemSet::on_update(GameState::ConsoleOpenedState)
                .with_system(game_loop.system())
                .with_system(commands::commands_handler.system()),
        );
    }
}

pub struct TicTacToeData {
    pub has_seen_tutorial: bool,
}

pub fn start_game(
    cg_data: &mut ResMut<ConsoleGamesData>,
) {
    cg_data.loaded_game = GameList::TicTacToe;
    info!("Starting tictactoe game");
}

pub fn game_loop(
    cg_data: Res<ConsoleGamesData>,
    mut ttt_data: ResMut<TicTacToeData>,
    mut console_writer: EventWriter<PrintConsoleEvent>,
) {
    if cg_data.loaded_game != GameList::TicTacToe { return ; }

    if !ttt_data.has_seen_tutorial {
        console_writer.send(PrintConsoleEvent(game::display_tutorial()));
        ttt_data.has_seen_tutorial = true;
    }
}