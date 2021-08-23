mod interactable;
mod player;

use crate::debug::collider_debug_lines_system;
use bevy::prelude::*;
use bevy_prototype_debug_lines::*;
use bevy_rapier2d::prelude::*;

pub use self::{
    interactable::{InteractableComponent, InteractableInRangeEvent, InteractableType},
    player::PlayerComponent,
};

pub struct ApartmentPlugin;

pub const BACKGROUND_Z: f32 = 0.0;
pub const PLAYER_Z: f32 = 1.0;
pub const FOREGROUND_Z: f32 = 10.0;

impl Plugin for ApartmentPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
            .add_plugin(DebugLinesPlugin)
            .add_event::<interactable::InteractableInRangeEvent>()
            .add_startup_system(setup.system().label("apartment_setup"))
            .add_startup_system(player::spawn_player.system().after("apartment_setup"))
            .add_startup_system(
                interactable::spawn_furniture_system
                    .system()
                    .after("apartment_setup"),
            )
            .add_system(player::player_movement_system.system())
            .add_system(
                interactable::interactable_system
                    .system()
                    .label("interactable_in_range"),
            )
            .add_system(
                interactable::log_interactable_in_range_event_system
                    .system()
                    .after("interactable_in_range"),
            );

        if cfg!(debug_assertions) {
            app.add_system(collider_debug_lines_system.system());
        }
    }
}

/// Setup physics, camera, background, foreground, walls
fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut rapier_config: ResMut<RapierConfiguration>,
) {
    // setup rapier
    rapier_config.gravity = Vector::zeros();
    rapier_config.scale = 10.0;

    // create camera
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    // create background
    let texture_handle = asset_server.load("textures/apartment_background.png");
    commands
        .spawn()
        .insert_bundle(SpriteBundle {
            material: materials.add(texture_handle.into()),
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, BACKGROUND_Z)),
            ..Default::default()
        })
        .insert(Name::new("Background"));

    // create foreground
    let texture_handle = asset_server.load("textures/apartment_foreground.png");
    commands
        .spawn()
        .insert_bundle(SpriteBundle {
            material: materials.add(texture_handle.into()),
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, FOREGROUND_Z)),
            ..Default::default()
        })
        .insert(Name::new("Foreground"));

    // create walls
    // top wall
    commands
        .spawn()
        .insert_bundle(RigidBodyBundle {
            body_type: RigidBodyType::Static,
            position: Vec2::new(-5.3, 6.5).into(),
            ..Default::default()
        })
        .insert_bundle(ColliderBundle {
            shape: ColliderShape::cuboid(27.5, 1.0),
            material: ColliderMaterial {
                friction: 0.0,
                restitution: 0.0,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(RigidBodyPositionSync::Discrete)
        .insert(Name::new("Top Wall"));

    // bottom wall
    commands
        .spawn()
        .insert_bundle(RigidBodyBundle {
            body_type: RigidBodyType::Static,
            position: Vec2::new(-5.3, -24.5).into(),
            ..Default::default()
        })
        .insert_bundle(ColliderBundle {
            shape: ColliderShape::cuboid(27.5, 1.0),
            material: ColliderMaterial {
                friction: 0.0,
                restitution: 0.0,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(RigidBodyPositionSync::Discrete)
        .insert(Name::new("Bottom Wall"));

    // right wall
    commands
        .spawn()
        .insert_bundle(RigidBodyBundle {
            body_type: RigidBodyType::Static,
            position: Vec2::new(20.6, -9.0).into(),
            ..Default::default()
        })
        .insert_bundle(ColliderBundle {
            shape: ColliderShape::cuboid(1.0, 16.0),
            material: ColliderMaterial {
                friction: 0.0,
                restitution: 0.0,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(RigidBodyPositionSync::Discrete)
        .insert(Name::new("Right Wall"));

    // left wall
    commands
        .spawn()
        .insert_bundle(RigidBodyBundle {
            body_type: RigidBodyType::Static,
            position: Vec2::new(-31.6, -9.0).into(),
            ..Default::default()
        })
        .insert_bundle(ColliderBundle {
            shape: ColliderShape::cuboid(1.0, 16.0),
            material: ColliderMaterial {
                friction: 0.0,
                restitution: 0.0,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(RigidBodyPositionSync::Discrete)
        .insert(Name::new("Left Wall"));
}
