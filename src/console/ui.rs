use bevy::prelude::*;
use super::{ConsoleAnimation, ConsoleData};

pub struct LogsArea;
pub struct ConsoleUI;

pub fn build_ui(
    mut commands: Commands, 
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut state: ResMut<ConsoleData>,
    mut anim_data: ResMut<ConsoleAnimation>,
    window: Res<Windows>,
) {
    let _current_window = window.get_primary().unwrap();

    anim_data.current_position = Vec2::new(0.0, -200.0);
    anim_data.desired_position = Vec2::new(0.0, 0.0);

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
        transform: Transform::from_xyz(anim_data.current_position.x, anim_data.current_position.y, 1.0),
        global_transform: GlobalTransform::identity(),
        ..Default::default()
    };

    state.entity = Some(
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

pub fn apply_animation(
    mut console_query: Query<(&ConsoleUI, &mut Transform)>,
    mut anim_data: ResMut<ConsoleAnimation>,
    time: Res<Time>,
) {
    anim_data.current_position = anim_data.current_position.lerp(
        anim_data.desired_position, 
        0.5 * time.delta_seconds()
    );

    if let Ok((_, mut transform)) = console_query.single_mut() {
        info!("Desired: {:?} Current : {:?}", anim_data.desired_position, anim_data.current_position);
        transform.translation.x = anim_data.current_position.x;
        transform.translation.y = anim_data.current_position.y;
    }
}