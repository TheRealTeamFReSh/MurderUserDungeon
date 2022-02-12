mod commands;
pub mod event;
mod input;
mod ui;

use crate::apartment::player::decrease_stats;

use self::commands::should_run_cmd_handler;

use super::states::GameState;
use bevy::prelude::*;
use sysinfo::{System, SystemExt};

pub struct ConsolePlugin;

impl Plugin for ConsolePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<event::PrintConsoleEvent>()
            .add_event::<event::EnteredConsoleCommandEvent>()
            .add_system_set(
                SystemSet::on_enter(GameState::MainGame)
                    .with_system(ui::build_ui.label("build_terminal"))
                    .with_system(setup),
            )
            .add_system_set(
                SystemSet::on_enter(GameState::ConsoleOpenedState)
                    .with_system(ui::open_console)
                    .with_system(input::opening_console_sound),
            )
            .add_system_set(
                SystemSet::on_update(GameState::ConsoleOpenedState)
                    .with_system(input::handle_input_keys.label("send_console_input"))
                    .with_system(input::update_enter_command)
                    .with_system(ui::update_logs_area),
            )
            .add_system_set(
                SystemSet::on_update(GameState::ConsoleOpenedState)
                    .with_run_criteria(should_run_cmd_handler)
                    .with_system(commands::commands_handler)
                    .before("send_console_input"),
            )
            .add_system_set(
                SystemSet::on_update(GameState::ConsoleOpenedState).with_system(decrease_stats),
            )
            .add_system_to_stage(CoreStage::PostUpdate, ui::apply_animation)
            .add_system_set(
                SystemSet::on_exit(GameState::ConsoleOpenedState)
                    .with_system(ui::close_console)
                    .with_system(input::closing_console_sound),
            )
            .insert_resource(ConsoleData::default())
            .insert_resource(ConsoleAnimation {
                moving_speed: 15.0,
                ..Default::default()
            })
            .init_resource::<System>()
            .add_system(event::add_message_events_to_console)
            .add_system(input::trigger_open_console.after("check_interactables"));
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
