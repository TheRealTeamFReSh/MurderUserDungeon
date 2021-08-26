use crate::apartment::{
    player::{PlayerComponent, Sleepiness},
    InteractableType, PlayerInBedComponent,
};

use crate::states::GameState;
use bevy::prelude::*;

pub const SLEEP_TIME: f32 = 4.0;

/// Handles interacting with bed
pub fn interact_bed_system(
    mut commands: Commands,
    player_query: Query<&PlayerComponent>,
    sleepiness: Res<Sleepiness>,
    keyboard_input: Res<Input<KeyCode>>,
    mut app_state: ResMut<State<GameState>>,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    audio: Res<Audio>,
) {
    for player_component in player_query.iter() {
        if let Some(InteractableType::Bed) = player_component.interactable_in_range {
            if keyboard_input.just_pressed(KeyCode::E)
                && app_state.current() == &GameState::MainGame
            {
                if sleepiness.0 <= 50 {
                    info!("Sleeping in bed");
                    audio.play(asset_server.load("audio/get_in_bed.mp3"));
                    if app_state.current() == &GameState::MainGame {
                        app_state.set(GameState::PlayerSleepingState).unwrap();
                    }
                    super::spawn_player_in_bed(&mut commands, &asset_server, &mut materials);
                } else {
                    info!("Not tired");
                    // TODO: notify the player that he is not tired
                }
            }
        }
    }
}

// Tracks player sleeping
pub struct SleepingResource {
    pub sleep_timer: Timer,
}

/// Handles player sleeping
pub fn sleeping_system(
    mut commands: Commands,
    player_in_bed_query: Query<Entity, With<PlayerInBedComponent>>,
    mut app_state: ResMut<State<GameState>>,
    mut sleep_resource: ResMut<SleepingResource>,
    mut sleepiness: ResMut<Sleepiness>,
    time: Res<Time>,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
) {
    if app_state.current() == &GameState::PlayerSleepingState {
        sleep_resource.sleep_timer.tick(time.delta());

        // wake up player when timer finished
        if sleep_resource.sleep_timer.just_finished() {
            sleep_resource.sleep_timer.reset();
            if app_state.current() == &GameState::PlayerSleepingState {
                app_state.set(GameState::MainGame).unwrap();
                super::despawn_player_in_bed(&mut commands, &player_in_bed_query);
                audio.play(asset_server.load("audio/get_out_bed.mp3"));
                sleepiness.0 = 100;
                info!("Player woke up");
            }
        }
    }
}
