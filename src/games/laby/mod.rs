pub mod walls;

use bevy::prelude::*;

pub struct LabyrinthGamePlugin;

impl Plugin for LabyrinthGamePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(setup.system());
    }
}

fn setup() {
    info!("Loading LabyrinthGamePlugin");
}

pub fn start_game() {
    info!("Starting laby game");
}