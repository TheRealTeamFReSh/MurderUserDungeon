mod animation;
mod bed;
mod door;
mod interactable;
pub mod player;

use crate::{apartment::player::decrease_stats, debug::collider_debug_lines_system};
use bevy::prelude::*;
use bevy_prototype_debug_lines::*;
use bevy_rapier2d::prelude::*;
use ron::de::from_bytes;

pub use self::{
    animation::BasicAnimationComponent,
    interactable::{InteractableComponent, InteractableType, InteractablesResource},
    player::{PlayerComponent, PLAYER_SPRITE_SCALE},
};

pub struct ApartmentPlugin;

pub const BACKGROUND_Z: f32 = 0.0;
pub const HALLWAY_COVER_Z: f32 = 1.0;
pub const PLAYER_IN_BED_Z: f32 = 0.9;
pub const NPC_Z: f32 = 4.0;
pub const PLAYER_Z: f32 = 5.0;
pub const FOREGROUND_Z: f32 = 10.0;
pub const INTERACTABLE_ICON_Z: f32 = 11.0;

impl Plugin for ApartmentPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
            .add_plugin(DebugLinesPlugin)
            .insert_resource(player::Health(100))
            .insert_resource(player::Hunger(100))
            .insert_resource(player::Sleepiness(100))
            .insert_resource(player::PeePeePooPoo(100))
            .insert_resource(player::StatsTimer(Timer::from_seconds(1.0, true)))
            .insert_resource(bed::SleepingResource {
                sleep_timer: Timer::from_seconds(bed::SLEEP_TIME, false),
            })
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
            )
            .add_system(decrease_stats.system())
            .add_system(
                bed::interact_bed_system
                    .system()
                    .after("check_interactables"),
            )
            .add_system(bed::sleeping_system.system())
            .add_system(player::hide_player_system.system());

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
            position: Vec2::new(12.0, 11.7).into(),
            ..Default::default()
        })
        .insert_bundle(ColliderBundle {
            shape: ColliderShape::cuboid(30.0, 6.2),
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
            position: Vec2::new(-33.3, 11.7).into(),
            ..Default::default()
        })
        .insert_bundle(ColliderBundle {
            shape: ColliderShape::cuboid(7.0, 6.2),
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

    // hallway wall
    commands
        .spawn()
        .insert_bundle(RigidBodyBundle {
            body_type: RigidBodyType::Static,
            position: Vec2::new(0.0, 25.0).into(),
            ..Default::default()
        })
        .insert_bundle(ColliderBundle {
            shape: ColliderShape::cuboid(40.0, 1.0),
            material: ColliderMaterial {
                friction: 0.0,
                restitution: 0.0,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(RigidBodyPositionSync::Discrete)
        .insert(Name::new("Hallway Back Wall"));

    // hallway right wall
    commands
        .spawn()
        .insert_bundle(RigidBodyBundle {
            body_type: RigidBodyType::Static,
            position: Vec2::new(41.5, 21.5).into(),
            ..Default::default()
        })
        .insert_bundle(ColliderBundle {
            shape: ColliderShape::cuboid(1.0, 4.0),
            material: ColliderMaterial {
                friction: 0.0,
                restitution: 0.0,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(RigidBodyPositionSync::Discrete)
        .insert(Name::new("Hallway Right Wall"));

    // hallway left wall
    commands
        .spawn()
        .insert_bundle(RigidBodyBundle {
            body_type: RigidBodyType::Static,
            position: Vec2::new(-41.5, 21.5).into(),
            ..Default::default()
        })
        .insert_bundle(ColliderBundle {
            shape: ColliderShape::cuboid(1.0, 4.0),
            material: ColliderMaterial {
                friction: 0.0,
                restitution: 0.0,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(RigidBodyPositionSync::Discrete)
        .insert(Name::new("Hallway Left Wall"));
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

pub struct PlayerInBedComponent;

pub fn spawn_player_in_bed(
    commands: &mut Commands,
    asset_server: &AssetServer,
    materials: &mut Assets<ColorMaterial>,
) {
    // create background
    let texture_handle = asset_server.load("textures/player_in_bed.png");
    commands
        .spawn()
        .insert(PlayerInBedComponent)
        .insert_bundle(SpriteBundle {
            material: materials.add(texture_handle.into()),
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, PLAYER_IN_BED_Z)),
            ..Default::default()
        })
        .insert(Name::new("Player in Bed"));
}

pub fn despawn_player_in_bed(
    commands: &mut Commands,
    player_in_bed_query: &Query<Entity, With<PlayerInBedComponent>>,
) {
    for player_in_bed in player_in_bed_query.iter() {
        commands.entity(player_in_bed).despawn();
    }
}
