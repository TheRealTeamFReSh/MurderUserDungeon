mod ui;
mod input;

use bevy::prelude::*;
use super::states::GameState;

pub struct ConsolePlugin;

impl Plugin for ConsolePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_startup_system(setup.system())
            .add_system_set(
                SystemSet::on_enter(GameState::ConsoleOpenedState)
                    .with_system(ui::build_ui.system()),
            )
            .add_system_set(
                SystemSet::on_update(GameState::ConsoleOpenedState)
                    .with_system(input::handle_logs_area.system())
                    .with_system(update_enter_command.system()),
            )
            .add_system_to_stage(
                CoreStage::PostUpdate,
                ui::apply_animation.system(),
            )
            .add_system_set(
                SystemSet::on_exit(GameState::ConsoleOpenedState)
                    .with_system(ui::destroy_ui.system()),
            )
            .insert_resource(ConsoleData::default())
            .insert_resource(ConsoleAnimation {
                moving_speed: 5.0,
                ..Default::default()
            })
            .add_system(trigger_open_console.system());
    }
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(UiCameraBundle::default());
}

fn trigger_open_console(
    keyboard_input: Res<Input<KeyCode>>,
    mut app_state: ResMut<State<GameState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Return) {
        match app_state.current() {
            GameState::MainGame => {
                app_state.set(GameState::ConsoleOpenedState).unwrap();
                info!("Console opened");
            } 

            _ => {
                app_state.set(GameState::MainGame).unwrap();
                info!("Console closed");
            }
        }
    }
}

#[derive(Default)]
pub struct ConsoleData {
    pub enter_command: String,
    pub entity: Option<Entity>,
}

#[derive(Default)]
pub struct ConsoleAnimation {
    pub start_position: Vec2,
    pub end_position: Vec2,
    pub moving_speed: f64,
    pub time_to_move: f64,
    pub start_time: f64,
}

fn update_enter_command(
    mut enter_command_text: Query<&mut Text, With<ui::LogsArea>>,
    state: Res<ConsoleData>,
    asset_server: Res<AssetServer>,
    time: Res<Time>,
) {
    let mut text = enter_command_text.single_mut().unwrap();
    text.sections = vec![];

    let mut to_show = String::from(" >  ");
    to_show.push_str(&state.enter_command);
    
    if (time.seconds_since_startup() * 3.0) as u64 % 2 == 0 {
        to_show.push('_');
    }

    text.sections.push(TextSection {
        value: to_show,
        style: TextStyle {
            font: asset_server.load("fonts/FiraSans-Medium.ttf"),
            font_size: 20.,
            color: Color::WHITE,
        },
    });
}