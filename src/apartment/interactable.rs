use crate::apartment::player::PlayerComponent;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

/// Types of interactable items
#[derive(Clone, Debug)]
pub enum InteractableType {
    Desk,
    Bed,
}

/// Stores data specific interactable item
pub struct InteractableComponent {
    interactable_type: InteractableType,
    range: f32,
}

/// Spawn collider, rigidbody, and possible interactable component for furniture
pub fn spawn_furniture_system(mut commands: Commands) {
    //spawn desk
    commands
        .spawn()
        .insert(InteractableComponent {
            interactable_type: InteractableType::Desk,
            range: 70.0,
        })
        .insert_bundle(RigidBodyBundle {
            body_type: RigidBodyType::Static,
            position: Vec2::new(-27.0, -19.0).into(),
            ..Default::default()
        })
        .insert_bundle(ColliderBundle {
            shape: ColliderShape::cuboid(3.0, 5.0),
            material: ColliderMaterial {
                friction: 0.0,
                restitution: 0.0,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(RigidBodyPositionSync::Discrete)
        .insert(Name::new("Desk"));

    // spawn bed
    commands
        .spawn()
        .insert(InteractableComponent {
            interactable_type: InteractableType::Bed,
            range: 60.0,
        })
        .insert_bundle(RigidBodyBundle {
            body_type: RigidBodyType::Static,
            position: Vec2::new(11.0, 5.0).into(),
            ..Default::default()
        })
        .insert_bundle(ColliderBundle {
            shape: ColliderShape::cuboid(8.5, 4.5),
            material: ColliderMaterial {
                friction: 0.0,
                restitution: 0.0,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(RigidBodyPositionSync::Discrete)
        .insert(Name::new("Bed"));
}

/// Event for sending event if interactable in range
pub struct InteractableInRangeEvent(InteractableType);

/// Sends event if interactable item in range
pub fn interactable_system(
    mut ev_interactable_in_range: EventWriter<InteractableInRangeEvent>,
    interactable_query: Query<(&InteractableComponent, &Transform)>,
    player_query: Query<&Transform, With<PlayerComponent>>,
) {
    for player_transform in player_query.iter() {
        for (interactable_component, interactable_transform) in interactable_query.iter() {
            let interactable_position: Vec2 = interactable_transform.translation.into();
            let player_position: Vec2 = player_transform.translation.into();

            // get distance between player and interactable
            let distance = interactable_position.distance(player_position);

            // send event if distance is less than interactable's range
            if distance < interactable_component.range {
                ev_interactable_in_range.send(InteractableInRangeEvent(
                    interactable_component.interactable_type.clone(),
                ));
            }
        }
    }
}

/// Print all interactable in range events
pub fn log_interactable_in_range_event_system(
    mut ev_interactable_in_range: EventReader<InteractableInRangeEvent>,
) {
    for _ev in ev_interactable_in_range.iter() {}
}
