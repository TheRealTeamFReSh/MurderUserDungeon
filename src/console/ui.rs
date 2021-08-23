use bevy::prelude::*;
use super::{ConsoleAnimation, ConsoleData};

pub struct LogsArea;
pub struct ConsoleUI;

pub fn build_ui(
    mut commands: Commands, 
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut anim_data: ResMut<ConsoleAnimation>,
    window: Res<Windows>,
) {
    let current_window = window.get_primary().unwrap();

    // move away the window
    anim_data.start_position = Vec2::new(0.0, -current_window.height());
    anim_data.end_position = anim_data.start_position;

    let background_component = NodeBundle {
        style: Style {
            size: Size::new(
                Val::Percent(100.0), 
                Val::Percent(100.0)),
            justify_content: JustifyContent::Center,
            flex_direction: FlexDirection::ColumnReverse,
            ..Default::default()
        },
        material: materials.add(Color::rgba_u8(5, 17, 0, 255).into()),
        ..Default::default()
    };

    commands.spawn_bundle(background_component)
        .insert(ConsoleUI {})
        .with_children(|parent| {
            // Logs Area
            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        size: Size::new(
                            Val::Percent(100.0), 
                            Val::Percent(90.0)),
                        ..Default::default()
                    },
                    material: materials.add(Color::rgba_u8(0, 0, 0, 0).into()),
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent
                        .spawn_bundle(TextBundle {
                            ..Default::default()
                        })
                        .insert(LogsArea);
                });
        });
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
    let new_position = anim_data.start_position.lerp(anim_data.end_position, value as f32);

    if data.is_opening && new_position.abs_diff_eq(anim_data.start_position, 1.0) {
        data.fully_opened = true;
    }

    if let Ok((_, mut style)) = console_query.single_mut() {
        style.position.top = Val::Px(new_position.y);
        style.position.left = Val::Px(new_position.x); 
    }
}