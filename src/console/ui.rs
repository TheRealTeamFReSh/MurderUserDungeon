use bevy::prelude::*;

pub struct LogsArea;

pub fn build_ui(
    mut commands: Commands, 
    mut materials: ResMut<Assets<ColorMaterial>>,
    window: Res<Windows>,
) {
    let _current_window = window.get_primary().unwrap();

    commands.spawn_bundle(UiCameraBundle::default());

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