use bevy::{input::keyboard::KeyboardInput, prelude::*};
use rand::Rng;

use super::event::EnteredConsoleCommandEvent;
use super::{ui, ConsoleData};
use crate::apartment::{InteractableType, PlayerComponent};
use crate::games::ConsoleGamesData;
use crate::states::GameState;
use crate::vulnerability::{BoolVulnerabilityType, VulnerabilityResource};

pub fn trigger_open_console(
    player_query: Query<&PlayerComponent>,
    keyboard_input: Res<Input<KeyCode>>,
    mut app_state: ResMut<State<GameState>>,
    cg_data: Res<ConsoleGamesData>,
    mut vuln_res: ResMut<VulnerabilityResource>,
) {
    for player_component in player_query.iter() {
        if let Some(InteractableType::Desk) = player_component.interactable_in_range {
            if keyboard_input.just_pressed(KeyCode::E)
                && app_state.current() == &GameState::MainGame
            {
                app_state.push(GameState::ConsoleOpenedState).unwrap();
                #[cfg(debug_assertions)]
                info!("Console opened");
            }
        }
    }

    if (app_state.current() == &GameState::ConsoleOpenedState)
        && keyboard_input.just_pressed(KeyCode::Escape)
    {
        app_state.pop().unwrap();
        #[cfg(debug_assertions)]
        info!("Console closed");
        if cg_data.has_won_laby {
            #[cfg(debug_assertions)]
            info!("Has won death");
            *vuln_res
                .bool_vulnerabilities
                .get_mut(&BoolVulnerabilityType::LabyrinthWinning)
                .unwrap() = true;
        }
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
    if !data.fully_opened {
        return;
    }

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
                    KeyCode::Space => data.enter_command.push(' '),
                    KeyCode::Tab => data.enter_command.push_str("  "),
                    KeyCode::Comma => data.enter_command.push(','),
                    KeyCode::Colon => data.enter_command.push(':'),
                    KeyCode::Semicolon => data.enter_command.push(';'),
                    KeyCode::Apostrophe => data.enter_command.push('\''),
                    KeyCode::At => data.enter_command.push('@'),
                    KeyCode::LBracket => data.enter_command.push('['),
                    KeyCode::RBracket => data.enter_command.push(']'),
                    KeyCode::Minus | KeyCode::NumpadSubtract => data.enter_command.push('-'),
                    KeyCode::Period | KeyCode::NumpadDecimal => data.enter_command.push('.'),
                    KeyCode::Asterisk | KeyCode::NumpadMultiply => data.enter_command.push('*'),
                    KeyCode::Slash | KeyCode::NumpadDivide => data.enter_command.push('/'),
                    KeyCode::Plus | KeyCode::NumpadAdd => data.enter_command.push('+'),
                    KeyCode::Key0 | KeyCode::Numpad0 => data.enter_command.push('0'),
                    KeyCode::Key1 | KeyCode::Numpad1 => data.enter_command.push('1'),
                    KeyCode::Key2 | KeyCode::Numpad2 => data.enter_command.push('2'),
                    KeyCode::Key3 | KeyCode::Numpad3 => data.enter_command.push('3'),
                    KeyCode::Key4 | KeyCode::Numpad4 => data.enter_command.push('4'),
                    KeyCode::Key5 | KeyCode::Numpad5 => data.enter_command.push('5'),
                    KeyCode::Key6 | KeyCode::Numpad6 => data.enter_command.push('6'),
                    KeyCode::Key7 | KeyCode::Numpad7 => data.enter_command.push('7'),
                    KeyCode::Key8 | KeyCode::Numpad8 => data.enter_command.push('8'),
                    KeyCode::Key9 | KeyCode::Numpad9 => data.enter_command.push('9'),

                    KeyCode::LShift
                    | KeyCode::RShift
                    | KeyCode::Escape
                    | KeyCode::LAlt
                    | KeyCode::RAlt
                    | KeyCode::LControl
                    | KeyCode::RControl
                    | KeyCode::F1
                    | KeyCode::Up
                    | KeyCode::Down
                    | KeyCode::Right
                    | KeyCode::Left
                    | KeyCode::F2
                    | KeyCode::F3
                    | KeyCode::F4
                    | KeyCode::F5
                    | KeyCode::F6
                    | KeyCode::F7
                    | KeyCode::F8
                    | KeyCode::F9
                    | KeyCode::F10
                    | KeyCode::F11
                    | KeyCode::F12
                    | KeyCode::Insert
                    | KeyCode::Delete
                    | KeyCode::Grave
                    | KeyCode::Backslash => {}

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

    if state.enter_command.len() > 144 {
        let trimmed_command = state.enter_command[..144].to_string();
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
