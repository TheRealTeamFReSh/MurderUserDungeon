mod ui;

use bevy::prelude::*;

use crate::states::GameState;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(setup.system());
        app.add_startup_system(ui::build_ui.system());
        app.add_system_set(SystemSet::on_update(GameState::MainMenu).with_system(start_game.system()));
    }
}

// Basic setup function in which setting up the camera and so on
fn setup() {
    info!("Starting MainMenu Plugin");
}

// Temporary command (Return key) to start the game
fn start_game(
    mut app_state: ResMut<State<GameState>>,
    keyboard_input: Res<Input<KeyCode>>, 
) {
    if keyboard_input.just_pressed(KeyCode::Return) {
        app_state.set(GameState::MainGame).unwrap();
    }
}