use crate::{apartment::PLAYER_Z, states::GameState};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

/// Stores core attributes of player
pub struct PlayerComponent {
    pub speed: f32,
}

/// Spawns a player
pub fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // spawn player
    let texture_handle = asset_server.load("textures/player.png");
    commands
        .spawn()
        .insert_bundle(SpriteBundle {
            material: materials.add(texture_handle.into()),
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, PLAYER_Z)),
            ..Default::default()
        })
        .insert_bundle(RigidBodyBundle {
            body_type: RigidBodyType::Dynamic,
            mass_properties: RigidBodyMassPropsFlags::ROTATION_LOCKED.into(),
            position: Vec2::new(10.0, 0.0).into(),
            ..Default::default()
        })
        .insert_bundle((
            RigidBodyPositionSync::Discrete,
            Name::new("Player"),
            PlayerComponent { speed: 1.5 },
        ))
        .with_children(|parent| {
            parent.spawn().insert_bundle(ColliderBundle {
                shape: ColliderShape::cuboid(3.0, 1.0),
                position: Vec2::new(0.0, -3.8).into(),
                material: ColliderMaterial {
                    friction: 0.0,
                    restitution: 0.0,
                    ..Default::default()
                },
                ..Default::default()
            });
        });
}

// Move player by modifying velocity with input
pub fn player_movement_system(
    keyboard_input: Res<Input<KeyCode>>,
    rapier_config: Res<RapierConfiguration>,
    mut player_info: Query<(&PlayerComponent, &mut RigidBodyVelocity)>,
    app_state: Res<State<GameState>>,
) {
    // if we are not playing the game prevent the player from moving
    if app_state.current() != &GameState::MainGame { return; }

    for (player, mut rb_vels) in player_info.iter_mut() {
        // get key presses
        let up = keyboard_input.pressed(KeyCode::W) || keyboard_input.pressed(KeyCode::Up);
        let down = keyboard_input.pressed(KeyCode::S) || keyboard_input.pressed(KeyCode::Down);
        let left = keyboard_input.pressed(KeyCode::A) || keyboard_input.pressed(KeyCode::Left);
        let right = keyboard_input.pressed(KeyCode::D) || keyboard_input.pressed(KeyCode::Right);

        // convert to axis multipliers
        let x_axis = -(left as i8) + right as i8;
        let y_axis = -(down as i8) + up as i8;

        // handle movement in x direction
        if x_axis != 0 {
            // accelerate to the player's maximum speed stat
            rb_vels.linvel.x = player.speed * (x_axis as f32) * rapier_config.scale;
        } else {
            rb_vels.linvel.x = 0.0;
        }

        // handle movement in y direction
        if y_axis != 0 {
            // accelerate to the player's maximum speed stat
            rb_vels.linvel.y = player.speed * (y_axis as f32) * rapier_config.scale;
        } else {
            rb_vels.linvel.y = 0.0;
        }
    }
}

pub struct Hunger(pub u32);
pub struct Sleepiness(pub u32);
pub struct PeePeePooPoo(pub u32);
pub struct Health(pub u32);
pub struct StatsTimer(pub Timer);

pub fn decrease_stats(
    mut hunger: ResMut<Hunger>,
    mut sleepiness: ResMut<Sleepiness>,
    mut peepeepoopoo: ResMut<PeePeePooPoo>,
    mut timer: ResMut<StatsTimer>,
    time: Res<Time>,
) {
    timer.0.tick(time.delta());
    if timer.0.finished() {
        let hunger_reduction = 5;
        if hunger.0 >= hunger_reduction {
            hunger.0 -= hunger_reduction
        } else {
            hunger.0 = 0
        };
        let sleepiness_reduction = 5;
        if sleepiness.0 >= sleepiness_reduction {
            sleepiness.0 -= sleepiness_reduction;
        } else {
            sleepiness.0 = 0
        }
        let peepeepoopoo_reduction = 5;
        if peepeepoopoo.0 >= peepeepoopoo_reduction {
            peepeepoopoo.0 -= peepeepoopoo_reduction;
        } else {
            peepeepoopoo.0 = 0
        }
        info!(
            "Hunger: {}, sleepiness: {}, peepeepoopoo: {}",
            hunger.0, sleepiness.0, peepeepoopoo.0
        );
    }
}
