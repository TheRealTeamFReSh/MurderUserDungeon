use bevy::prelude::*;
use rand::Rng;
use serde::Deserialize;
use std::collections::HashMap;

use super::PlayerComponent;

/// Tag for basic (1 row) animation
#[derive(Component)]
pub struct BasicAnimationComponent;

/// Animate basic (1 row) animations
pub fn basic_sprite_animation_system(
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<
        (&mut Timer, &mut TextureAtlasSprite, &Handle<TextureAtlas>),
        With<BasicAnimationComponent>,
    >,
) {
    for (mut timer, mut sprite, texture_atlas_handle) in query.iter_mut() {
        timer.tick(time.delta());
        if timer.finished() {
            let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
            sprite.index = (sprite.index as usize + 1) % texture_atlas.textures.len();
        }
    }
}

/// Animate a characters (people, includes player)
pub fn animate_character_system(
    time: Res<Time>,
    character_animations: Res<CharacterAnimationResource>,
    mut animation_query: Query<(&mut CharacterAnimationComponent, &mut TextureAtlasSprite)>,
) {
    for (mut character_animation, mut sprite) in animation_query.iter_mut() {
        character_animation.timer.tick(time.delta());

        if character_animation.timer.finished() {
            let animation_idxs =
                character_animations.animations[&character_animation.animation_type];
            if sprite.index == animation_idxs.1 as usize {
                sprite.index = animation_idxs.0 as usize;
            } else {
                sprite.index += 1;
            }
        }
    }
}

/// Stores data about character animations frames (data/character_animations.ron)
#[derive(Deserialize)]
pub struct CharacterAnimationResource {
    // start and end indexes of animations
    pub animations: HashMap<CharacterAnimationType, (u32, u32, f32)>,
}

/// Types of character animations
#[derive(Deserialize, Hash, PartialEq, Eq, Clone)]
pub enum CharacterAnimationType {
    ForwardIdle,
    LeftIdle,
    BackwardIdle,
    RightIdle,
    ForwardMove,
    LeftMove,
    BackwardMove,
    RightMove,
}

impl CharacterAnimationType {
    fn is_idle(&self) -> bool {
        matches!(
            self,
            CharacterAnimationType::ForwardIdle
                | CharacterAnimationType::BackwardIdle
                | CharacterAnimationType::LeftIdle
                | CharacterAnimationType::RightIdle
        )
    }
}

/// Used for tracking animations of a character entity
#[derive(Component)]
pub struct CharacterAnimationComponent {
    pub timer: Timer,
    pub animation_type: CharacterAnimationType,
}

pub struct WalkingSound {
    pub timer: Timer,
    pub first_time: bool,
}

pub fn player_walking_sound_system(
    mut ws_res: ResMut<WalkingSound>,
    mut player_query: Query<&mut CharacterAnimationComponent, With<PlayerComponent>>,
    audio: Res<Audio>,
    asset_server: Res<AssetServer>,
    time: Res<Time>,
) {
    for char_animation in player_query.iter_mut() {
        if !char_animation.animation_type.is_idle() {
            ws_res.timer.tick(time.delta());
            if ws_res.timer.finished() || ws_res.first_time {
                if ws_res.first_time {
                    ws_res.first_time = false;
                    ws_res.timer.reset();
                }

                let index = rand::thread_rng().gen_range(1..8);
                audio.play(
                    asset_server.load(format!("audio/footstep/footstep-{}.mp3", index).as_str()),
                );
            }
        } else {
            ws_res.timer.reset();
            ws_res.first_time = true;
        }
    }
}
