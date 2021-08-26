use crate::{
    apartment::{
        animation::{
            CharacterAnimationComponent, CharacterAnimationResource, CharacterAnimationType,
        },
        PLAYER_Z,
    },
    states::GameState,
};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use super::InteractableType;

pub const PLAYER_SPRITE_SCALE: f32 = 2.0;

/// Stores core attributes of player
#[derive(Debug)]
pub struct PlayerComponent {
    pub speed: f32,
    pub interactable_in_range: Option<InteractableType>,
}

/// Spawns a player
pub fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    character_animations: Res<CharacterAnimationResource>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    // spawn player
    let character_starting_animation = CharacterAnimationType::ForwardIdle;
    let texture_handle = asset_server.load("textures/player_spritesheet.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(32.0, 46.0), 6, 8);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    let sprite_transform = Transform {
        translation: Vec3::new(0.0, 0.0, PLAYER_Z),
        scale: Vec3::new(PLAYER_SPRITE_SCALE, PLAYER_SPRITE_SCALE, 0.0),
        ..Default::default()
    };

    commands
        .spawn()
        .insert(PlayerComponent {
            speed: 1.5,
            interactable_in_range: None,
        })
        .insert(CharacterAnimationComponent {
            timer: Timer::from_seconds(
                character_animations.animations[&character_starting_animation].2,
                true,
            ),
            animation_type: character_starting_animation.clone(),
        })
        .insert_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            transform: sprite_transform,
            sprite: TextureAtlasSprite {
                index: character_animations.animations[&character_starting_animation].0,
                ..Default::default()
            },
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
            PlayerComponent {
                speed: 1.5,
                interactable_in_range: None,
            },
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

/// Set the player's animation based on what the player is doing
pub fn set_player_animation_system(
    keyboard_input: Res<Input<KeyCode>>,
    character_animations: Res<CharacterAnimationResource>,
    mut player_query: Query<
        (
            &mut CharacterAnimationComponent,
            &mut TextureAtlasSprite,
            &RigidBodyVelocity,
        ),
        With<PlayerComponent>,
    >,
) {
    for (mut character_animation, mut sprite, rb_vels) in player_query.iter_mut() {
        let mut restart_animation = false;

        // set to idle animation if velocity is 0 and key is released
        if rb_vels.linvel.x == 0.0 && rb_vels.linvel.y == 0.0 {
            if keyboard_input.just_released(KeyCode::A)
                || keyboard_input.just_released(KeyCode::Left)
            {
                character_animation.animation_type = CharacterAnimationType::LeftIdle;
                restart_animation = true;
            } else if keyboard_input.just_released(KeyCode::D)
                || keyboard_input.just_released(KeyCode::Right)
            {
                character_animation.animation_type = CharacterAnimationType::RightIdle;
                restart_animation = true;
            } else if keyboard_input.just_released(KeyCode::W)
                || keyboard_input.just_released(KeyCode::Up)
            {
                character_animation.animation_type = CharacterAnimationType::BackwardIdle;
                restart_animation = true;
            } else if keyboard_input.just_released(KeyCode::S)
                || keyboard_input.just_released(KeyCode::Down)
            {
                character_animation.animation_type = CharacterAnimationType::ForwardIdle;
                restart_animation = true;
            }
        }
        // set to move animation if key pressed
        if keyboard_input.just_pressed(KeyCode::A) || keyboard_input.just_pressed(KeyCode::Left) {
            character_animation.animation_type = CharacterAnimationType::LeftMove;
            restart_animation = true;
        } else if keyboard_input.just_pressed(KeyCode::D)
            || keyboard_input.just_pressed(KeyCode::Right)
        {
            character_animation.animation_type = CharacterAnimationType::RightMove;
            restart_animation = true;
        } else if keyboard_input.just_pressed(KeyCode::W)
            || keyboard_input.just_pressed(KeyCode::Up)
        {
            character_animation.animation_type = CharacterAnimationType::BackwardMove;
            restart_animation = true;
        } else if keyboard_input.just_pressed(KeyCode::S)
            || keyboard_input.just_pressed(KeyCode::Down)
        {
            character_animation.animation_type = CharacterAnimationType::ForwardMove;
            restart_animation = true;
        }

        // if animation changed restart the timer, sprite, and set animation type
        if restart_animation {
            let animation_data =
                character_animations.animations[&character_animation.animation_type];
            sprite.index = animation_data.0;
            character_animation.timer = Timer::from_seconds(animation_data.2, true);
        }
    }
}

// Hide player in designated states
pub fn hide_player_system(
    app_state: Res<State<GameState>>,
    mut player_sprite_query: Query<&mut TextureAtlasSprite, With<PlayerComponent>>,
) {
    for mut sprite in player_sprite_query.iter_mut() {
        sprite.color = if app_state.current() == &GameState::PlayerSleepingState
            || app_state.current() == &GameState::GameOverState(true)
        {
            Color::NONE
        } else {
            Color::WHITE
        }
    }
}

// Move player by modifying velocity with input
pub fn player_movement_system(
    keyboard_input: Res<Input<KeyCode>>,
    rapier_config: Res<RapierConfiguration>,
    mut player_info: Query<(&PlayerComponent, &mut RigidBodyVelocity)>,
    app_state: Res<State<GameState>>,
) {
    // if we are not playing the game prevent the player from moving
    if app_state.current() != &GameState::MainGame {
        return;
    }

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
