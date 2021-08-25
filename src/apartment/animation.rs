use bevy::prelude::*;
use serde::Deserialize;
use std::collections::HashMap;

/// Tag for basic (1 row) animation
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
            sprite.index = ((sprite.index as usize + 1) % texture_atlas.textures.len()) as u32;
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
            if sprite.index == animation_idxs.1 {
                sprite.index = animation_idxs.0;
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

/// Used for tracking animations of a character entity
pub struct CharacterAnimationComponent {
    pub timer: Timer,
    pub animation_type: CharacterAnimationType,
}
