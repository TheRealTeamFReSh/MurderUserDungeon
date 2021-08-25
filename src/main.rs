mod apartment;
mod console;
mod debug;
mod games;
mod misc;
mod states;
mod vulnerability;

use bevy::{app::AppExit, prelude::*, window::WindowMode};
#[allow(unused_imports)]
use bevy_inspector_egui::WorldInspectorPlugin;
use bevy_kira_audio::AudioPlugin;

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
        .add_plugin(console::ConsolePlugin)
        .add_plugin(AudioPlugin)
        .add_plugin(apartment::ApartmentPlugin)
        .add_plugin(vulnerability::VulnerabilityPlugin)
        .add_plugin(games::ConsoleGamesPlugin)
        .add_plugin(misc::game_over::GameOverPlugin)
        //.add_plugin(WorldInspectorPlugin::new())
        .add_state(states::GameState::MainGame)
        .add_system(exit_on_esc_system.system())
        .run();
}

pub fn exit_on_esc_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut app_exit_events: EventWriter<AppExit>,
    app_state: Res<State<states::GameState>>,
) {
    if app_state.current() == &states::GameState::MainGame
        && keyboard_input.just_pressed(KeyCode::Escape)
    {
        app_exit_events.send(AppExit);
    }
}
