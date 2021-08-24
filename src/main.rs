mod apartment;
mod console;
mod states;
mod main_menu;
mod debug;

use bevy::{app::AppExit, prelude::*, window::WindowMode};
use bevy_inspector_egui::WorldInspectorPlugin;
use bevy_kira_audio::AudioPlugin;
use states::GameState;

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
        .add_state(GameState::MainMenu)

        .add_plugins(DefaultPlugins)
        .add_plugin(AudioPlugin)
        .add_plugin(console::ConsolePlugin)
        .add_plugin(apartment::ApartmentPlugin)
        .add_plugin(main_menu::MainMenuPlugin)
        .add_plugin(WorldInspectorPlugin::new())
        
        .add_system(exit_on_esc_system.system())
        .run();
}

pub fn exit_on_esc_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut app_exit_events: EventWriter<AppExit>,
    app_state: Res<State<GameState>>,
) {
    if app_state.current() == &GameState::MainGame && keyboard_input.just_pressed(KeyCode::Escape) {
        app_exit_events.send(AppExit);
    }
}


