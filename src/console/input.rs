use bevy::{input::keyboard::KeyboardInput, prelude::*};

use super::{ConsoleData};
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

pub fn handle_logs_area(
    mut state: ResMut<ConsoleData>,
    mut evr_keys: EventReader<KeyboardInput>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    for ev in evr_keys.iter() {
        if ev.state.is_pressed() {
            if let Some(key_code) = ev.key_code {
                match key_code {
                    KeyCode::Back => {
                        if !state.enter_command.is_empty() {
                            state.enter_command.pop();
                        }
                    }
                    KeyCode::Space => {
                        state.enter_command.push(' ');
                    }
                    KeyCode::Tab => {
                        state.enter_command.push_str("  ");
                    }
                    KeyCode::Comma => {
                        state.enter_command.push(',');
                    }
                    KeyCode::Colon => {
                        state.enter_command.push(':');
                    }
                    KeyCode::Semicolon => {
                        state.enter_command.push(';');
                    }
                    KeyCode::Period => {
                        state.enter_command.push('.');
                    }
                    KeyCode::Asterisk => {
                        state.enter_command.push('*');
                    }
                    KeyCode::Slash => {
                        state.enter_command.push('/');
                    }
                    KeyCode::Apostrophe => {
                        state.enter_command.push('\'');
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