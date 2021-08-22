mod ui;

use bevy::{input::keyboard::KeyboardInput, prelude::*};

pub struct ConsolePlugin;

impl Plugin for ConsolePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .insert_resource(ConsoleState::default())
            .add_startup_system(setup.system())
            .add_startup_system(ui::build_ui.system())
            .add_system(open_console.system())
            .add_system(handle_logs_area.system())
            .add_system(update_enter_command.system());
    }
}

fn open_console(
    keyboard_input: Res<Input<KeyCode>>,
    mut state: ResMut<ConsoleState>,
) {
    if keyboard_input.just_pressed(KeyCode::E) {
        state.opened = !state.opened;
        info!("Console opened: {}", state.opened);
    }
}

#[derive(Default)]
pub struct ConsoleState {
    pub opened: bool,
    pub enter_command: String,
}

fn setup(
    mut state: ResMut<ConsoleState>,
) {
    info!("Starting ConsolePlugin");
    state.opened = true;
}

fn handle_logs_area(
    mut state: ResMut<ConsoleState>,
    mut evr_keys: EventReader<KeyboardInput>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    for ev in evr_keys.iter() {
        if ev.state.is_pressed() {
            if let Some(key_code) = ev.key_code {
                match key_code {
                    KeyCode::Back => {
                        if state.enter_command.len() != 0 {
                            state.enter_command.pop();
                        }
                    }
                    KeyCode::Space => {
                        state.enter_command.push_str(" ");
                    }
                    KeyCode::Tab => {
                        state.enter_command.push_str("  ");
                    }
                    KeyCode::Comma => {
                        state.enter_command.push_str(",");
                    }
                    KeyCode::Colon => {
                        state.enter_command.push_str(":");
                    }
                    KeyCode::Semicolon => {
                        state.enter_command.push_str(";");
                    }
                    KeyCode::Period => {
                        state.enter_command.push_str(".");
                    }
                    KeyCode::Asterisk => {
                        state.enter_command.push_str("*");
                    }
                    KeyCode::Slash => {
                        state.enter_command.push_str("/");
                    }
                    KeyCode::Apostrophe => {
                        state.enter_command.push_str("'");
                    }

                    KeyCode::LShift | KeyCode::RShift => {}

                    KeyCode::Return => {
                        state.enter_command.clear();
                    }
                    _ => {
                        let key_code_str = if keyboard_input.pressed(KeyCode::LShift) || 
                            keyboard_input.pressed(KeyCode::RShift) {
                            format!("{:?}", key_code).to_uppercase()
                        } else {
                            format!("{:?}", key_code).to_lowercase()
                        };

                        trace!("Pressed key: {:?}", key_code_str);
                        state.enter_command.push_str(&key_code_str);
                    } 
                }
            }
        }
    }
}

fn update_enter_command(
    mut enter_command_text: Query<&mut Text, With<ui::LogsArea>>,
    state: Res<ConsoleState>,
    asset_server: Res<AssetServer>,
) {
    let mut text = enter_command_text.single_mut().unwrap();
    text.sections = vec![];
    text.sections.push(TextSection {
        value: state.enter_command.clone(),
        style: TextStyle {
            font: asset_server.load("fonts/FiraSans-Medium.ttf"),
            font_size: 20.,
            color: Color::WHITE,
        },
    });
}