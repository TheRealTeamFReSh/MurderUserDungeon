use crate::apartment::{
    player::PlayerComponent, InteractableComponent, InteractableType, InteractablesResource,
};

use crate::states::GameState;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

/// Handles opening and closing of door
pub fn interact_door_system(
    mut commands: Commands,
    player_query: Query<&PlayerComponent>,
    interactable_query: Query<(Entity, &InteractableComponent)>,
    interactable_icon_query: Query<Entity, With<super::interactable::InteractableIconComponent>>,
    interactables_resource: Res<InteractablesResource>,
    keyboard_input: Res<Input<KeyCode>>,
    app_state: Res<State<GameState>>,
) {
    for player_component in player_query.iter() {
        if let Some(InteractableType::ClosedDoor) = player_component.interactable_in_range {
            if keyboard_input.just_pressed(KeyCode::E)
                && app_state.current() == &GameState::MainGame
            {
                info!("Door opened");
                // despawn the closed door
                for (interactable_entity, interactable_component) in interactable_query.iter() {
                    if let InteractableType::ClosedDoor = interactable_component.interactable_type {
                        commands.entity(interactable_entity).despawn();
                        super::interactable::despawn_interactable_icons(
                            &mut commands,
                            &interactable_icon_query,
                        );
                    }
                }

                // spawn an open door
                spawn_open_door(&mut commands, &interactables_resource);
            }
        } else if let Some(InteractableType::OpenDoor) = player_component.interactable_in_range {
            if keyboard_input.just_pressed(KeyCode::E)
                && app_state.current() == &GameState::MainGame
            {
                info!("Door closed");
                // despawn the open door
                for (interactable_entity, interactable_component) in interactable_query.iter() {
                    if let InteractableType::OpenDoor = interactable_component.interactable_type {
                        commands.entity(interactable_entity).despawn();
                        super::interactable::despawn_interactable_icons(
                            &mut commands,
                            &interactable_icon_query,
                        );
                    }
                }

                // spawn an open door
                spawn_closed_door(&mut commands, &interactables_resource);
            }
        }
    }
}

/// Spawns a closed door (with collider)
pub fn spawn_closed_door(commands: &mut Commands, interactables_resource: &InteractablesResource) {
    // spawn door
    let interactable_type = InteractableType::ClosedDoor;
    let interactable_data = &interactables_resource.interactables[&interactable_type];
    commands
        .spawn()
        .insert(InteractableComponent {
            interactable_type,
            range: interactable_data.range,
        })
        .insert_bundle(RigidBodyBundle {
            body_type: RigidBodyType::Static,
            position: interactable_data.position.into(),
            ..Default::default()
        })
        .insert_bundle(ColliderBundle {
            shape: ColliderShape::cuboid(
                interactable_data.collider_size.x,
                interactable_data.collider_size.y,
            ),
            material: ColliderMaterial {
                friction: 0.0,
                restitution: 0.0,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(RigidBodyPositionSync::Discrete)
        .insert(Name::new("Closed Door"));
}

/// Spawns an open door (no collider)
pub fn spawn_open_door(commands: &mut Commands, interactables_resource: &InteractablesResource) {
    // spawn door
    let interactable_type = InteractableType::OpenDoor;
    let interactable_data = &interactables_resource.interactables[&interactable_type];
    commands
        .spawn()
        .insert(InteractableComponent {
            interactable_type,
            range: interactable_data.range,
        })
        .insert_bundle(RigidBodyBundle {
            body_type: RigidBodyType::Static,
            position: interactable_data.position.into(),
            ..Default::default()
        })
        .insert(RigidBodyPositionSync::Discrete)
        .insert(Name::new("Open Door"));
}
