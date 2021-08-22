use bevy::prelude::*;
use super::ConsoleData;

pub struct LogsArea;

pub fn build_ui(
    mut commands: Commands, 
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut state: ResMut<ConsoleData>,
    window: Res<Windows>,
) {
    let _current_window = window.get_primary().unwrap();

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

    state.entity = Some(
        commands.spawn_bundle(background_component)
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
            }).id()
    );
}

pub fn destroy_ui(
    mut commands: Commands,
    mut state: ResMut<ConsoleData>,
) {
    // if there is an entity spawned

    if let Some(entity) = state.entity {
        commands.entity(entity).despawn_recursive();
        state.entity = None;
    }
}