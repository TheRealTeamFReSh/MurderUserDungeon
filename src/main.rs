mod console;
mod states;

use bevy_inspector_egui::WorldInspectorPlugin;
use bevy::{prelude::*, window::WindowMode};

fn main() {
    App::build()
        .insert_resource(WindowDescriptor {
            width: 800.0,
            height: 600.0,
            title: "RustyJam".to_string(),
            vsync: false,
            mode: WindowMode::Windowed,
            resizable: true,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_system(bevy::input::system::exit_on_esc_system.system())
        .add_plugin(console::ConsolePlugin)
        .add_plugin(WorldInspectorPlugin::new())
        .add_state(states::GameState::MainGame)
        .run();
}

