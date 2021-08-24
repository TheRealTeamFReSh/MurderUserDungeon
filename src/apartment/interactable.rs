use crate::apartment::{animation::BasicAnimationComponent, player::PlayerComponent};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use serde::Deserialize;
use std::collections::HashMap;

use super::INTERACTABLE_ICON_Z;

const INTERACTABLE_ICON_SPRITE_SCALE: f32 = 2.5;
const INTERACTABLE_ICON_Y_OFFSET: f32 = 4.0;
const INTERACTABLE_ICON_FRAME_TIME: f32 = 0.15;

/// Types of interactable items
#[derive(Deserialize, Hash, Clone, Debug, PartialEq, Eq)]
pub enum InteractableType {
    Desk,
    Bed,
}

/// Stores data about sizes, locations, and ranges of interactables
#[derive(Deserialize)]
pub struct InteractablesResource {
    interactables: HashMap<InteractableType, InteractableData>,
}

#[derive(Deserialize)]
pub struct InteractableData {
    pub position: Vec2,
    pub collider_size: Vec2,
    pub range: f32,
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
#[derive(PartialEq)]
pub struct InteractableInRangeEvent(pub InteractableType);

pub fn check_interactables_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    interactables_resource: Res<InteractablesResource>,
    interactable_query: Query<(&InteractableComponent, &Transform)>,
    mut player_query: Query<(&Transform, &mut PlayerComponent)>,
    interactable_icon_query: Query<Entity, With<InteractableIconComponent>>,
    rapier_config: Res<RapierConfiguration>,
) {
    for (player_transform, mut player_component) in player_query.iter_mut() {
        let mut interactable_in_range: Option<InteractableType> = None;
        for (interactable_component, interactable_transform) in interactable_query.iter() {
            let interactable_position: Vec2 = interactable_transform.translation.into();
            let player_position: Vec2 = player_transform.translation.into();

            // get distance between player and interactable
            let distance = interactable_position.distance(player_position);

            // send event if distance is less than interactable's range
            if distance < interactable_component.range {
                interactable_in_range = Some(interactable_component.interactable_type.clone());
            }
        }

        let old_interactable = player_component.interactable_in_range.clone();
        player_component.interactable_in_range = interactable_in_range.clone();
        // spawn interact icon
        if old_interactable != interactable_in_range {
            if let Some(interactable_type) = interactable_in_range {
                // spawn interact icon
                spawn_interact_icon(
                    &interactable_type,
                    &mut commands,
                    &interactables_resource,
                    &asset_server,
                    &mut texture_atlases,
                    &rapier_config,
                );
            } else {
                // despawn all interact icons
                for interactable_icon_entity in interactable_icon_query.iter() {
                    commands.entity(interactable_icon_entity).despawn();
                }
            }
        }
    }
}

/// Tag for interactable icons
pub struct InteractableIconComponent;

/// Spawn an interactable icon
fn spawn_interact_icon(
    interactable_type: &InteractableType,
    commands: &mut Commands,
    interactables_resource: &InteractablesResource,
    asset_server: &AssetServer,
    texture_atlases: &mut Assets<TextureAtlas>,
    rapier_config: &RapierConfiguration,
) {
    let interactable_data = &interactables_resource.interactables[interactable_type];

    let texture_handle = asset_server.load("textures/e_key_press.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(16.0, 16.0), 6, 1);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    let sprite_transform = Transform {
        translation: Vec3::new(
            interactable_data.position.x * rapier_config.scale,
            (interactable_data.position.y * rapier_config.scale)
                + (INTERACTABLE_ICON_Y_OFFSET * rapier_config.scale),
            INTERACTABLE_ICON_Z,
        ),
        scale: Vec3::new(
            INTERACTABLE_ICON_SPRITE_SCALE,
            INTERACTABLE_ICON_SPRITE_SCALE,
            0.0,
        ),
        ..Default::default()
    };

    commands
        .spawn()
        .insert(InteractableIconComponent)
        .insert(BasicAnimationComponent)
        .insert(Timer::from_seconds(INTERACTABLE_ICON_FRAME_TIME, true))
        .insert_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            transform: sprite_transform,
            ..Default::default()
        })
        .insert(Name::new("Interactable Icon"));
}

#[allow(dead_code)]
/// Print all interactable in range events
pub fn log_interactable_in_range_event_system(
    mut ev_interactable_in_range: EventReader<InteractableInRangeEvent>,
) {
    for ev in ev_interactable_in_range.iter() {
        info!("{:?} in range of player", ev.0);
    }
}
