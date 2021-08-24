use bevy::prelude::*;

use crate::states::GameState;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(setup.system());
        app.add_system_set(SystemSet::on_update(GameState::MainMenu).with_system(start_game.system()));
    }
}

fn setup() {
    info!("Starting MainMenu Plugin");
}

fn start_game(
    mut app_state: ResMut<State<GameState>>,
    keyboard_input: Res<Input<KeyCode>>, 
) {
    if keyboard_input.just_pressed(KeyCode::Return) {
        app_state.set(GameState::MainGame).unwrap();
    }
}