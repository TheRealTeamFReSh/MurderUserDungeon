// Configure clippy for Bevy usage
#![allow(clippy::type_complexity)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::too_many_lines)]
#![allow(clippy::must_use_candidate)]
#![allow(clippy::needless_pass_by_value)]
#![allow(clippy::enum_glob_use)]

mod apartment;
mod console;
mod debug;
mod games;
mod hud;
mod main_menu;
mod misc;
mod npcs;
mod states;
mod vulnerability;

use bevy::{app::AppExit, prelude::*, window::WindowMode};
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
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(hud::Plugin)
        .add_plugin(npcs::NPCsPlugin)
        .add_plugin(misc::day_cycle::DayCyclePlugin)
        .add_plugin(main_menu::Plugin)
        .add_state(states::GameState::MainMenu)
        .add_startup_system(spawn_ui_camera.system())
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

fn spawn_ui_camera(mut commands: Commands) {
    commands.spawn_bundle(UiCameraBundle::default());
}
