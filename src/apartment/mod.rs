mod animation;
mod bed;
pub mod door;
mod interactable;
pub mod phone;
pub mod player;
mod toilet;

use bevy::prelude::*;
//use bevy_prototype_debug_lines::*;
use bevy_rapier2d::prelude::*;
use rand::Rng;
use ron::de::from_bytes;

use crate::{apartment::player::decrease_stats, misc::day_cycle, states::GameState};

pub use self::{
    animation::BasicAnimationComponent,
    interactable::{InteractableComponent, InteractableType, InteractablesResource},
    player::{PlayerComponent, PLAYER_SPRITE_SCALE},
};

pub struct ApartmentPlugin;

pub const BACKGROUND_Z: f32 = 0.0;
pub const HALLWAY_COVER_Z: f32 = 1.0;
pub const PLAYER_IN_BED_Z: f32 = 2.0;
pub const PIZZA_Z: f32 = 2.0;
pub const NPC_Z: f32 = 4.0;
pub const PLAYER_Z: f32 = 5.0;
pub const FOREGROUND_Z: f32 = 10.0;
pub const INTERACTABLE_ICON_Z: f32 = 11.0;
pub const LIGHTING_Z: f32 = 10.5;
pub const PEEPHOLE_Z: f32 = 10.2;

impl Plugin for ApartmentPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
            .insert_resource(player::Health(rand::thread_rng().gen_range(80.0..=100.0)))
            .insert_resource(player::Hunger(rand::thread_rng().gen_range(80.0..=100.0)))
            .insert_resource(player::Sleepiness(
                rand::thread_rng().gen_range(80.0..=100.0),
            ))
            .insert_resource(player::PeePeePooPoo(
                rand::thread_rng().gen_range(80.0..=100.0),
            ))
            .insert_resource(player::StatsTimer(Timer::from_seconds(1.0, true)))
            .insert_resource(bed::SleepingResource {
                sleep_timer: Timer::from_seconds(bed::SLEEP_TIME, false),
            })
            .insert_resource(toilet::PeeingResource {
                pee_timer: Timer::from_seconds(toilet::PEE_TIME, false),
            })
            .insert_resource(phone::OrderingPizzaResource {
                call_timer: Timer::from_seconds(phone::CALL_TIME, false),
            })
            .insert_resource(phone::EatingResource {
                eating_timer: Timer::from_seconds(phone::EAT_TIME, false),
            })
            .insert_resource(phone::PizzaDeliveryResource {
                status: phone::PizzaDeliveryStatus::Unordered,
                delivery_timer: Timer::from_seconds(
                    day_cycle::DAY_LENGTH * ((phone::DELIVERY_TIME) / 24.0),
                    true,
                ),
                at_door_timer: Timer::from_seconds(
                    day_cycle::DAY_LENGTH * ((phone::AT_DOOR_TIME) / 24.0),
                    true,
                ),
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
            .insert_resource(animation::WalkingSound {
                first_time: true,
                timer: Timer::from_seconds(0.4, true),
            })
            .add_system_set(
                SystemSet::on_enter(GameState::MainGame)
                    .with_system(setup.label("apartment_setup"))
                    .with_system(player::spawn_player.after("apartment_setup"))
                    .with_system(interactable::spawn_furniture_system.after("apartment_setup")),
            )
            .add_system_set(
                SystemSet::on_update(GameState::MainGame)
                    .with_system(player::player_movement_system.label("player_movement"))
                    .with_system(
                        interactable::check_interactables_system.label("check_interactables"),
                    )
                    .with_system(
                        player::set_player_animation_system
                            .after("player_movement")
                            .label("set_player_animation"),
                    )
                    .with_system(door::interact_door_system.after("check_interactables"))
                    .with_system(bed::interact_bed_system.after("check_interactables"))
                    .with_system(toilet::interact_toilet_system.after("check_interactables"))
                    .with_system(phone::interact_pizza_system.after("check_interactables"))
                    .with_system(phone::interact_phone_system.after("check_interactables"))
                    .with_system(animation::player_walking_sound_system.after("player_movement"))
                    .with_system(decrease_stats),
            );

        app.add_system_set(
            SystemSet::on_update(GameState::PeepholeOpenedState).with_system(decrease_stats),
        );

        app.add_system_set(
            SystemSet::on_update(GameState::PlayerEatingState).with_system(decrease_stats),
        );

        app.add_system_set(
            SystemSet::on_update(GameState::PlayerOrderingPizzaState).with_system(decrease_stats),
        );

        app.add_system_set(
            SystemSet::on_update(GameState::PlayerSleepingState).with_system(decrease_stats),
        );

        app.add_system_set(
            SystemSet::on_update(GameState::PlayerPeeingState).with_system(decrease_stats),
        );

        app.add_system_set(
            SystemSet::on_update(GameState::PlayerHidingState).with_system(decrease_stats),
        );

        app.add_system(animation::basic_sprite_animation_system);
        app.add_system(bed::sleeping_system)
            .add_system(toilet::peeing_system)
            .add_system(phone::ordering_pizza_system)
            .add_system(phone::eating_system)
            .add_system(phone::pizza_delivery_system)
            .add_system(player::hide_player_system)
            .add_system(door::exit_peephole_system)
            .add_system(bed::exit_hiding_system.label("exit_hiding"));
        app.add_system(animation::animate_character_system.after("set_player_animation"));

        /*
        if cfg!(debug_assertions) {
            app.add_system_set(
                SystemSet::on_update(GameState::MainGame).with_system(collider_debug_lines_system),
            );
        }
        */
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
    //let texture_handle = asset_server.load("textures/apartment_background.png");
    commands
        .spawn()
        .insert_bundle(SpriteBundle {
            texture: asset_server.load("textures/apartment_background.png"),
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, BACKGROUND_Z)),
            ..Default::default()
        })
        .insert(Name::new("Background"));

    spawn_hallway_cover(&mut commands, &asset_server, &mut materials);

    // create foreground
    //let texture_handle = asset_server.load("textures/apartment_foreground.png");
    commands
        .spawn()
        .insert_bundle(SpriteBundle {
            texture: asset_server.load("textures/apartment_foreground.png"),
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, FOREGROUND_Z)),
            ..Default::default()
        })
        .insert(Name::new("Foreground"));

    // create walls
    // top wall right
    commands
        .spawn()
        .insert_bundle(RigidBodyBundle {
            body_type: RigidBodyType::Static.into(),
            position: Vec2::new(12.0, 11.7).into(),
            ..Default::default()
        })
        .insert_bundle(ColliderBundle {
            shape: ColliderShape::cuboid(30.0, 6.2).into(),
            material: ColliderMaterial {
                friction: 0.0,
                restitution: 0.0,
                ..Default::default()
            }
            .into(),
            ..Default::default()
        })
        .insert(RigidBodyPositionSync::Discrete)
        .insert(Name::new("Top-Right Wall"));

    // top wall left
    commands
        .spawn()
        .insert_bundle(RigidBodyBundle {
            body_type: RigidBodyType::Static.into(),
            position: Vec2::new(-33.3, 11.7).into(),
            ..Default::default()
        })
        .insert_bundle(ColliderBundle {
            shape: ColliderShape::cuboid(7.0, 6.2).into(),
            material: ColliderMaterial {
                friction: 0.0,
                restitution: 0.0,
                ..Default::default()
            }
            .into(),
            ..Default::default()
        })
        .insert(RigidBodyPositionSync::Discrete)
        .insert(Name::new("Top-Left Wall"));

    // bottom wall
    commands
        .spawn()
        .insert_bundle(RigidBodyBundle {
            body_type: RigidBodyType::Static.into(),
            position: Vec2::new(-5.3, -24.5).into(),
            ..Default::default()
        })
        .insert_bundle(ColliderBundle {
            shape: ColliderShape::cuboid(27.5, 1.0).into(),
            material: ColliderMaterial {
                friction: 0.0,
                restitution: 0.0,
                ..Default::default()
            }
            .into(),
            ..Default::default()
        })
        .insert(RigidBodyPositionSync::Discrete)
        .insert(Name::new("Bottom Wall"));

    // right wall
    commands
        .spawn()
        .insert_bundle(RigidBodyBundle {
            body_type: RigidBodyType::Static.into(),
            position: Vec2::new(20.6, -9.0).into(),
            ..Default::default()
        })
        .insert_bundle(ColliderBundle {
            shape: ColliderShape::cuboid(1.0, 16.0).into(),
            material: ColliderMaterial {
                friction: 0.0,
                restitution: 0.0,
                ..Default::default()
            }
            .into(),
            ..Default::default()
        })
        .insert(RigidBodyPositionSync::Discrete)
        .insert(Name::new("Right Wall"));

    // left wall
    commands
        .spawn()
        .insert_bundle(RigidBodyBundle {
            body_type: RigidBodyType::Static.into(),
            position: Vec2::new(-31.6, -9.0).into(),
            ..Default::default()
        })
        .insert_bundle(ColliderBundle {
            shape: ColliderShape::cuboid(1.0, 16.0).into(),
            material: ColliderMaterial {
                friction: 0.0,
                restitution: 0.0,
                ..Default::default()
            }
            .into(),
            ..Default::default()
        })
        .insert(RigidBodyPositionSync::Discrete)
        .insert(Name::new("Left Wall"));

    // hallway wall
    commands
        .spawn()
        .insert_bundle(RigidBodyBundle {
            body_type: RigidBodyType::Static.into(),
            position: Vec2::new(0.0, 25.0).into(),
            ..Default::default()
        })
        .insert_bundle(ColliderBundle {
            shape: ColliderShape::cuboid(40.0, 1.0).into(),
            material: ColliderMaterial {
                friction: 0.0,
                restitution: 0.0,
                ..Default::default()
            }
            .into(),
            ..Default::default()
        })
        .insert(RigidBodyPositionSync::Discrete)
        .insert(Name::new("Hallway Back Wall"));

    // hallway right wall
    commands
        .spawn()
        .insert_bundle(RigidBodyBundle {
            body_type: RigidBodyType::Static.into(),
            position: Vec2::new(41.5, 21.5).into(),
            ..Default::default()
        })
        .insert_bundle(ColliderBundle {
            shape: ColliderShape::cuboid(1.0, 4.0).into(),
            material: ColliderMaterial {
                friction: 0.0,
                restitution: 0.0,
                ..Default::default()
            }
            .into(),
            ..Default::default()
        })
        .insert(RigidBodyPositionSync::Discrete)
        .insert(Name::new("Hallway Right Wall"));

    // hallway left wall
    commands
        .spawn()
        .insert_bundle(RigidBodyBundle {
            body_type: RigidBodyType::Static.into(),
            position: Vec2::new(-41.5, 21.5).into(),
            ..Default::default()
        })
        .insert_bundle(ColliderBundle {
            shape: ColliderShape::cuboid(1.0, 4.0).into(),
            material: ColliderMaterial {
                friction: 0.0,
                restitution: 0.0,
                ..Default::default()
            }
            .into(),
            ..Default::default()
        })
        .insert(RigidBodyPositionSync::Discrete)
        .insert(Name::new("Hallway Left Wall"));

    // bathroom wall
    commands
        .spawn()
        .insert_bundle(RigidBodyBundle {
            body_type: RigidBodyType::Static.into(),
            position: Vec2::new(11.1, -10.8).into(),
            ..Default::default()
        })
        .insert_bundle(ColliderBundle {
            shape: ColliderShape::cuboid(8.2, 6.2).into(),
            material: ColliderMaterial {
                friction: 0.0,
                restitution: 0.0,
                ..Default::default()
            }
            .into(),
            ..Default::default()
        })
        .insert(RigidBodyPositionSync::Discrete)
        .insert(Name::new("Bathroom Wall"));

    // spawn dining table
    commands
        .spawn()
        .insert_bundle(RigidBodyBundle {
            body_type: RigidBodyType::Static.into(),
            position: Vec2::new(-9.6, -10.1).into(),
            ..Default::default()
        })
        .insert_bundle(ColliderBundle {
            shape: ColliderShape::cuboid(4.5, 3.1).into(),
            material: ColliderMaterial {
                friction: 0.0,
                restitution: 0.0,
                ..Default::default()
            }
            .into(),
            ..Default::default()
        })
        .insert(RigidBodyPositionSync::Discrete)
        .insert(Name::new("Dining Table"));

    // spawn desk chair
    commands
        .spawn()
        .insert_bundle(RigidBodyBundle {
            body_type: RigidBodyType::Static.into(),
            position: Vec2::new(-22.0, -22.0).into(),
            ..Default::default()
        })
        .insert_bundle(ColliderBundle {
            shape: ColliderShape::cuboid(3.0, 2.5).into(),
            material: ColliderMaterial {
                friction: 0.0,
                restitution: 0.0,
                ..Default::default()
            }
            .into(),
            ..Default::default()
        })
        .insert(RigidBodyPositionSync::Discrete)
        .insert(Name::new("Desk Chair"));
}

#[derive(Component)]
pub struct HallwayCoverComponent;

pub fn spawn_hallway_cover(
    commands: &mut Commands,
    asset_server: &AssetServer,
    materials: &mut Assets<ColorMaterial>,
) {
    // create background
    //let texture_handle = asset_server.load("textures/apartment_hallway_cover.png");
    commands
        .spawn()
        .insert(HallwayCoverComponent)
        .insert_bundle(SpriteBundle {
            texture: asset_server.load("textures/apartment_hallway_cover.png"),
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

#[derive(Component)]
pub struct PlayerInBedComponent;

pub fn spawn_player_in_bed(
    commands: &mut Commands,
    asset_server: &AssetServer,
    materials: &mut Assets<ColorMaterial>,
) {
    // create background
    //let texture_handle = asset_server.load("textures/player_in_bed.png");
    commands
        .spawn()
        .insert(PlayerInBedComponent)
        .insert_bundle(SpriteBundle {
            texture: asset_server.load("textures/player_in_bed.png"),
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

#[derive(Component)]
pub struct PizzaComponent;

pub fn spawn_pizza(
    commands: &mut Commands,
    asset_server: &AssetServer,
    materials: &mut Assets<ColorMaterial>,
) {
    // create background
    //let texture_handle = asset_server.load("textures/pizza.png");
    commands
        .spawn()
        .insert(PizzaComponent)
        .insert_bundle(SpriteBundle {
            texture: asset_server.load("textures/pizza.png"),
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, PIZZA_Z)),
            ..Default::default()
        })
        .insert(Name::new("Pizza"));
}

pub fn despawn_pizza(commands: &mut Commands, pizza_query: &Query<Entity, With<PizzaComponent>>) {
    for pizza in pizza_query.iter() {
        commands.entity(pizza).despawn();
    }
}

#[derive(Component)]
pub struct PeepholeComponent;

pub fn spawn_peephole(
    path: &str,
    commands: &mut Commands,
    asset_server: &AssetServer,
    materials: &mut Assets<ColorMaterial>,
) {
    // create background
    //let texture_handle = asset_server.load(path);
    commands
        .spawn()
        .insert(PeepholeComponent)
        .insert_bundle(SpriteBundle {
            texture: asset_server.load(path),
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, PEEPHOLE_Z)),
            ..Default::default()
        })
        .insert(Name::new("Peephole"));
}

pub fn despawn_peepholes(
    commands: &mut Commands,
    peephole_query: &Query<Entity, With<PeepholeComponent>>,
) {
    for peephole in peephole_query.iter() {
        commands.entity(peephole).despawn();
    }
}

#[derive(Component)]
pub struct HidingScreenComponent;

pub fn spawn_hiding_screen(
    commands: &mut Commands,
    asset_server: &AssetServer,
    materials: &mut Assets<ColorMaterial>,
) {
    commands
        .spawn()
        .insert(HidingScreenComponent)
        .insert_bundle(SpriteBundle {
            texture: asset_server.load("textures/hiding.png"),
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, PEEPHOLE_Z)),
            ..Default::default()
        })
        .insert(Name::new("Hiding Screen"));
}

pub fn despawn_hiding_screen(
    commands: &mut Commands,
    hiding_screen_query: &Query<Entity, With<HidingScreenComponent>>,
) {
    for hiding_screen in hiding_screen_query.iter() {
        commands.entity(hiding_screen).despawn();
    }
}
