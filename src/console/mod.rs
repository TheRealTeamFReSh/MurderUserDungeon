mod commands;
mod event;
mod input;
mod ui;

use super::states::GameState;
use bevy::prelude::*;
use sysinfo::{System, SystemExt};

pub struct ConsolePlugin;

impl Plugin for ConsolePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_event::<event::PrintConsoleEvent>()
            .add_event::<event::EnteredConsoleCommandEvent>()
            .add_startup_system(ui::build_ui.system())
            .add_startup_system(setup.system())
            .add_system_set(
                SystemSet::on_enter(GameState::ConsoleOpenedState)
                    .with_system(ui::open_console.system()),
            )
            .add_system_set(
                SystemSet::on_update(GameState::ConsoleOpenedState)
                    .with_system(input::handle_input_keys.system())
                    .with_system(input::update_enter_command.system())
                    .with_system(ui::update_logs_area.system())
                    .with_system(commands::commands_handler.system()),
            )
            .add_system_to_stage(CoreStage::PostUpdate, ui::apply_animation.system())
            .add_system_set(
                SystemSet::on_exit(GameState::ConsoleOpenedState)
                    .with_system(ui::close_console.system()),
            )
            .insert_resource(ConsoleData::default())
            .insert_resource(ConsoleAnimation {
                moving_speed: 15.0,
                ..Default::default()
            })
            .init_resource::<System>()
            .add_system(event::add_message_events_to_console.system())
            .add_system(
                input::trigger_open_console
                    .system()
                    .after("check_interactables"),
            );
    }
}

#[derive(Default)]
pub struct ConsoleData {
    pub enter_command: String,
    pub is_opening: bool,
    pub fully_opened: bool,
    pub messages: Vec<String>,
}

#[derive(Default)]
pub struct ConsoleAnimation {
    pub start_position: Vec2,
    pub end_position: Vec2,
    pub moving_speed: f64,
    pub time_to_move: f64,
    pub start_time: f64,
}

fn setup(mut sys: ResMut<System>) {
    sys.refresh_all();
}
