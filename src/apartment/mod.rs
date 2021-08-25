mod animation;
mod door;
mod interactable;
mod player;

use crate::debug::collider_debug_lines_system;
use bevy::prelude::*;
use bevy_prototype_debug_lines::*;
use bevy_rapier2d::prelude::*;
use ron::de::from_bytes;

pub use self::{
    interactable::{InteractableComponent, InteractableType, InteractablesResource},
    player::PlayerComponent,
};

pub struct ApartmentPlugin;

pub const BACKGROUND_Z: f32 = 0.0;
pub const HALLWAY_COVER_Z: f32 = 1.0;
pub const PLAYER_Z: f32 = 5.0;
pub const FOREGROUND_Z: f32 = 10.0;
pub const INTERACTABLE_ICON_Z: f32 = 11.0;

impl Plugin for ApartmentPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
            .add_plugin(DebugLinesPlugin)
            .insert_resource(
                from_bytes::<animation::CharacterAnimationResource>(include_bytes!(
                    "../../data/character_animations.ron"
                ))
                .unwrap(),
            )
            .insert_resource(
                from_bytes::<interactable::InteractablesResource>(include_bytes!(
                    "../../data/interactables.ron"
                ))
                .unwrap(),
            )
            .add_startup_system(setup.system().label("apartment_setup"))
            .add_startup_system(player::spawn_player.system().after("apartment_setup"))
            .add_startup_system(
                interactable::spawn_furniture_system
                    .system()
                    .after("apartment_setup"),
            )
            .add_system(
                player::player_movement_system
                    .system()
                    .label("player_movement"),
            )
            .add_system(
                interactable::check_interactables_system
                    .system()
                    .label("check_interactables"),
            )
            .add_system(
                player::set_player_animation_system
                    .system()
                    .after("player_movement")
                    .label("set_player_animation"),
            )
            .add_system(
                animation::animate_character_system
                    .system()
                    .after("set_player_animation"),
            )
            .add_system(animation::basic_sprite_animation_system.system())
            .add_system(
                door::interact_door_system
                    .system()
                    .after("check_interactables"),
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

    spawn_hallway_cover(&mut commands, &asset_server, &mut materials);

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
    // top wall right
    commands
        .spawn()
        .insert_bundle(RigidBodyBundle {
            body_type: RigidBodyType::Static,
            position: Vec2::new(1.5, 6.5).into(),
            ..Default::default()
        })
        .insert_bundle(ColliderBundle {
            shape: ColliderShape::cuboid(19.5, 1.0),
            material: ColliderMaterial {
                friction: 0.0,
                restitution: 0.0,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(RigidBodyPositionSync::Discrete)
        .insert(Name::new("Top-Right Wall"));

    // top wall left
    commands
        .spawn()
        .insert_bundle(RigidBodyBundle {
            body_type: RigidBodyType::Static,
            position: Vec2::new(-28.5, 6.5).into(),
            ..Default::default()
        })
        .insert_bundle(ColliderBundle {
            shape: ColliderShape::cuboid(2.5, 1.0),
            material: ColliderMaterial {
                friction: 0.0,
                restitution: 0.0,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(RigidBodyPositionSync::Discrete)
        .insert(Name::new("Top-Left Wall"));

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

pub struct HallwayCoverComponent;

pub fn spawn_hallway_cover(
    commands: &mut Commands,
    asset_server: &AssetServer,
    materials: &mut Assets<ColorMaterial>,
) {
    // create background
    let texture_handle = asset_server.load("textures/apartment_hallway_cover.png");
    commands
        .spawn()
        .insert(HallwayCoverComponent)
        .insert_bundle(SpriteBundle {
            material: materials.add(texture_handle.into()),
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, HALLWAY_COVER_Z)),
            ..Default::default()
        })
        .insert(Name::new("Hallway Cover"));
}

pub fn despawn_hallway_cover(
    commands: &mut Commands,
    hallway_cover_query: &Query<Entity, With<HallwayCoverComponent>>,
) {
    for hallway_cover in hallway_cover_query.iter() {
        commands.entity(hallway_cover).despawn();
    }
}
