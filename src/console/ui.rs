use super::{commands::print_motd, event::PrintConsoleEvent, ConsoleAnimation, ConsoleData};
use bevy::prelude::*;

pub struct LogsArea;
pub struct CommandLineText;
pub struct ConsoleUI;

use sysinfo::System;

pub fn build_ui(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut anim_data: ResMut<ConsoleAnimation>,
    window: Res<Windows>,
    mut sys: ResMut<System>,
    mut console_writer: EventWriter<PrintConsoleEvent>,
) {
    let current_window = window.get_primary().unwrap();

    // move away the window
    anim_data.start_position = Vec2::new(0.0, -current_window.height());
    anim_data.end_position = anim_data.start_position;

    // building the background color
    let background_component = NodeBundle {
        style: Style {
            size: Size::new(
                Val::Px(current_window.width()),
                Val::Px(current_window.height()),
            ),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::FlexStart,
            padding: Rect {
                left: Val::Percent(2.0),
                ..Default::default()
            },
            flex_direction: FlexDirection::ColumnReverse,
            ..Default::default()
        },
        material: materials.add(Color::rgba_u8(5, 17, 0, 255).into()),
        ..Default::default()
    };

    let transparent_col = Color::rgba_u8(0, 0, 0, 0);

    // don't forget the UI camera
    commands.spawn_bundle(UiCameraBundle::default());
    commands
        .spawn_bundle(background_component)
        .insert(ConsoleUI {})
        .with_children(|parent| {
            //container
            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Px(0.75 * current_window.width()), Val::Percent(95.0)),
                        justify_content: JustifyContent::Center,
                        flex_direction: FlexDirection::ColumnReverse,
                        flex_wrap: FlexWrap::Wrap,
                        ..Default::default()
                    },
                    material: materials.add(transparent_col.into()),
                    ..Default::default()
                })
                .with_children(|parent| {
                    // logs area
                    parent
                        .spawn_bundle(NodeBundle {
                            style: Style {
                                size: Size::new(
                                    Val::Px(0.75 * current_window.width()),
                                    Val::Percent(90.0),
                                ),
                                justify_content: JustifyContent::FlexEnd,
                                flex_direction: FlexDirection::ColumnReverse,
                                flex_wrap: FlexWrap::Wrap,
                                ..Default::default()
                            },
                            material: materials.add(transparent_col.into()),
                            ..Default::default()
                        })
                        .with_children(|parent| {
                            parent
                                .spawn_bundle(TextBundle {
                                    ..Default::default()
                                })
                                .insert(LogsArea);
                        });

                    // command textbox area
                    parent
                        .spawn_bundle(NodeBundle {
                            style: Style {
                                size: Size::new(
                                    Val::Px(0.75 * current_window.width()),
                                    Val::Percent(10.0),
                                ),
                                flex_wrap: FlexWrap::Wrap,
                                ..Default::default()
                            },
                            material: materials.add(transparent_col.into()),
                            ..Default::default()
                        })
                        .with_children(|parent| {
                            parent
                                .spawn_bundle(TextBundle {
                                    style: Style {
                                        size: Size::new(
                                            Val::Px(0.75 * current_window.width()),
                                            Val::Percent(10.0),
                                        ),
                                        flex_wrap: FlexWrap::Wrap,
                                        ..Default::default()
                                    },
                                    ..Default::default()
                                })
                                .insert(CommandLineText);
                        });
                });
        });

    console_writer.send(PrintConsoleEvent(print_motd(&mut sys, false)));
}

pub fn open_console(
    mut anim_data: ResMut<ConsoleAnimation>,
    mut data: ResMut<ConsoleData>,
    time: Res<Time>,
    window: Res<Windows>,
) {
    let current_window = window.get_primary().unwrap();

    data.is_opening = true;

    anim_data.start_position = Vec2::new(0.0, -current_window.height());
    anim_data.end_position = Vec2::new(0.0, 0.0);
    anim_data.start_time = time.seconds_since_startup();
}

pub fn close_console(
    mut anim_data: ResMut<ConsoleAnimation>,
    mut data: ResMut<ConsoleData>,
    time: Res<Time>,
    window: Res<Windows>,
) {
    let current_window = window.get_primary().unwrap();

    data.fully_opened = false;
    data.is_opening = false;

    anim_data.end_position = Vec2::new(0.0, -current_window.height());
    anim_data.start_position = Vec2::new(0.0, 0.0);
    anim_data.start_time = time.seconds_since_startup();
}

pub fn apply_animation(
    mut console_query: Query<(&ConsoleUI, &mut Style)>,
    anim_data: Res<ConsoleAnimation>,
    mut data: ResMut<ConsoleData>,
    time: Res<Time>,
) {
    let delta_t = time.seconds_since_startup() - anim_data.start_time;
    let value = 1.0 - (-(delta_t * anim_data.moving_speed)).exp();
    let new_position = anim_data
        .start_position
        .lerp(anim_data.end_position, value as f32);

    if data.is_opening && new_position.abs_diff_eq(anim_data.start_position, 1.0) {
        data.fully_opened = true;
    }

    if let Ok((_, mut style)) = console_query.single_mut() {
        style.position.top = Val::Px(new_position.y);
        style.position.left = Val::Px(new_position.x);
    }
}

pub fn update_logs_area(
    data: Res<ConsoleData>,
    asset_server: Res<AssetServer>,
    mut logs_area_query: Query<&mut Text, With<LogsArea>>,
) {
    let sections = data
        .messages
        .iter()
        .flat_map(|msg| {
            let mut msg = msg.clone();
            msg.push('\n');

            std::array::IntoIter::new([TextSection {
                value: msg.clone(),
                style: TextStyle {
                    font: asset_server.load("fonts/VT323-Regular.ttf"),
                    font_size: 16.,
                    color: Color::rgba_u8(76, 207, 76, 255),
                },
            }])
        })
        .collect::<Vec<_>>();

    let mut text = logs_area_query.single_mut().unwrap();
    text.sections = sections;
}
