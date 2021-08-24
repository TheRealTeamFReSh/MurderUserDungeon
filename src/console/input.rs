use bevy::{input::keyboard::KeyboardInput, prelude::*};
use rand::Rng;

use super::event::EnteredConsoleCommandEvent;
use super::{ui, ConsoleData};
use crate::apartment::{InteractableType, PlayerComponent};
use crate::states::GameState;

pub fn trigger_open_console(
    player_query: Query<&PlayerComponent>,
    keyboard_input: Res<Input<KeyCode>>,
    mut app_state: ResMut<State<GameState>>,
) {

    for player_component in player_query.iter() {
        if let Some(InteractableType::Desk) = player_component.interactable_in_range {
            if keyboard_input.just_pressed(KeyCode::E)
                && app_state.current() == &GameState::MainGame
            {
                app_state.set(GameState::ConsoleOpenedState).unwrap();
                info!("Console opened");
            }
        }
    }

    if app_state.current() == &GameState::ConsoleOpenedState
        && keyboard_input.just_pressed(KeyCode::Escape)
    {
        app_state.set(GameState::MainGame).unwrap();
        info!("Console closed");
    }
}

pub fn handle_input_keys(
    mut data: ResMut<ConsoleData>,
    mut evr_keys: EventReader<KeyboardInput>,
    keyboard_input: Res<Input<KeyCode>>,
    mut ev_writer: EventWriter<EnteredConsoleCommandEvent>,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
) {
    // if the console is not open yet
    if !data.fully_opened { return; }

    for ev in evr_keys.iter() {
        if ev.state.is_pressed() {
            let random_key = rand::thread_rng().gen_range(1..10);
            audio.play(asset_server.load(format!("audio/keys/key-{}.mp3", random_key).as_str()));

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
                        // sending the command
                        ev_writer.send(EnteredConsoleCommandEvent(data.enter_command.clone()));
                        // clearing the input
                        data.enter_command.clear();
                    }
                    _ => {
                        let key_code_str = if keyboard_input.pressed(KeyCode::LShift)
                            || keyboard_input.pressed(KeyCode::RShift)
                        {
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
    mut state: ResMut<ConsoleData>,
    asset_server: Res<AssetServer>,
    time: Res<Time>,
) {
    let mut text = enter_command_text.single_mut().unwrap();
    text.sections = vec![];

    if state.enter_command.len() > 215 {
        let trimmed_command = state.enter_command[..215].to_string();
        state.enter_command = trimmed_command;
    }

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
