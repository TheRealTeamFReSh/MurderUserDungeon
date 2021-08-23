use bevy::{input::keyboard::KeyboardInput, prelude::*};

use super::{ConsoleData, ui};
use crate::states::GameState;
use crate::apartment::{InteractableInRangeEvent, InteractableType};

pub fn trigger_open_console(
    mut ev_in_range: EventReader<InteractableInRangeEvent>,
    keyboard_input: Res<Input<KeyCode>>,
    mut app_state: ResMut<State<GameState>>,
) {
    for InteractableInRangeEvent(inter_type) in ev_in_range.iter() {
        if inter_type == &InteractableType::Desk && keyboard_input.just_pressed(KeyCode::Return) && app_state.current() == &GameState::MainGame {
            app_state.set(GameState::ConsoleOpenedState).unwrap();
            info!("Console opened");
        }
    }

    if app_state.current() == &GameState::ConsoleOpenedState && keyboard_input.just_pressed(KeyCode::Escape) {
        app_state.set(GameState::MainGame).unwrap();
        info!("Console closed");
    }
}

pub fn handle_input_keys(
    mut data: ResMut<ConsoleData>,
    mut evr_keys: EventReader<KeyboardInput>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    for ev in evr_keys.iter() {
        if ev.state.is_pressed() {
            if let Some(key_code) = ev.key_code {
                match key_code {
                    KeyCode::Back => {
                        if !data.enter_command.is_empty() {
                            data.enter_command.pop();
                        }
                    }
                    KeyCode::Space => {
                        data.enter_command.push(' ');
                    }
                    KeyCode::Tab => {
                        data.enter_command.push_str("  ");
                    }
                    KeyCode::Comma => {
                        data.enter_command.push(',');
                    }
                    KeyCode::Colon => {
                        data.enter_command.push(':');
                    }
                    KeyCode::Semicolon => {
                        data.enter_command.push(';');
                    }
                    KeyCode::Period => {
                        data.enter_command.push('.');
                    }
                    KeyCode::Asterisk => {
                        data.enter_command.push('*');
                    }
                    KeyCode::Slash => {
                        data.enter_command.push('/');
                    }
                    KeyCode::Apostrophe => {
                        data.enter_command.push('\'');
                    }

                    KeyCode::LShift | KeyCode::RShift | KeyCode::Escape => {}

                    KeyCode::Return => {
                        if data.fully_opened {
                            // getting the command
                            let command = data.enter_command.clone();
                            data.messages.push(command);
                            // clearing the input
                            data.enter_command.clear();
                        }
                    }
                    _ => {
                        let key_code_str = if keyboard_input.pressed(KeyCode::LShift) || 
                            keyboard_input.pressed(KeyCode::RShift) {
                            format!("{:?}", key_code).to_uppercase()
                        } else {
                            format!("{:?}", key_code).to_lowercase()
                        };

                        trace!("Pressed key: {:?}", key_code_str);
                        data.enter_command.push_str(&key_code_str);
                    } 
                }
            }
        }
    }
}

pub fn update_enter_command(
    mut enter_command_text: Query<&mut Text, With<ui::CommandLineText>>,
    state: Res<ConsoleData>,
    asset_server: Res<AssetServer>,
    time: Res<Time>,
) {
    let mut text = enter_command_text.single_mut().unwrap();
    text.sections = vec![];

    let mut to_show = String::from(">  ");
    to_show.push_str(&state.enter_command);
    
    if (time.seconds_since_startup() * 3.0) as u64 % 2 == 0 {
        to_show.push('_');
    }

    text.sections.push(TextSection {
        value: to_show,
        style: TextStyle {
            font: asset_server.load("fonts/VT323-Regular.ttf"),
            font_size: 20.,
            color: Color::rgba_u8(102, 255, 102, 255),
        },
    });
}