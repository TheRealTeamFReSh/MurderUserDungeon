use crate::misc::day_cycle::DayCycleResource;
use crate::vulnerability::{spawn_npc, BoolVulnerabilityType, VulnerabilityResource};
use crate::{
    apartment::{
        interactable::despawn_interactable_icons,
        phone::{PizzaDeliveryResource, PizzaDeliveryStatus},
        player::{PlayerComponent, Sleepiness},
        InteractableComponent, InteractableType, PlayerInBedComponent,
    },
    misc::game_over::{GameOverData, GameOverReason},
};
use std::time::Duration;

use crate::states::GameState;
use bevy::prelude::*;

pub const SLEEP_TIME: f32 = 4.0;

/// Handles interacting with bed
pub fn interact_bed_system(
    mut commands: Commands,
    interactables_query: Query<&InteractableComponent>,
    mut vulnerability_resource: ResMut<VulnerabilityResource>,
    player_query: Query<&PlayerComponent>,
    sleepiness: Res<Sleepiness>,
    keyboard_input: Res<Input<KeyCode>>,
    mut app_state: ResMut<State<GameState>>,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    audio: Res<Audio>,
    interactable_icon_query: Query<Entity, With<super::interactable::InteractableIconComponent>>,
) {
    for player_component in player_query.iter() {
        if let Some(InteractableType::Bed) = player_component.interactable_in_range {
            if keyboard_input.just_pressed(KeyCode::E)
                && app_state.current() == &GameState::MainGame
            {
                if sleepiness.0 <= 50. {
                    #[cfg(debug_assertions)]
                    info!("Sleeping in bed");
                    audio.play(asset_server.load("audio/get_in_bed.mp3"));
                    if app_state.current() == &GameState::MainGame {
                        app_state.push(GameState::PlayerSleepingState).unwrap();
                    }
                    super::spawn_player_in_bed(&mut commands, &asset_server, &mut materials);

                    for interactable_component in interactables_query.iter() {
                        if InteractableType::OpenDoor == interactable_component.interactable_type {
                            *vulnerability_resource
                                .bool_vulnerabilities
                                .get_mut(&BoolVulnerabilityType::BedDoorLeftOpen)
                                .unwrap() = true;
                        }
                    }
                } else {
                    #[cfg(debug_assertions)]
                    info!("Not tired");
                    // TODO: notify the player that he is not tired
                }
            } else if keyboard_input.just_pressed(KeyCode::C)
                && app_state.current() == &GameState::MainGame
            {
                app_state.push(GameState::PlayerHidingState).unwrap();
                vulnerability_resource.is_hiding = true;

                despawn_interactable_icons(&mut commands, &interactable_icon_query);
                super::spawn_hiding_screen(&mut commands, &asset_server, &mut materials);
                info!("Hiding under bed.")
            }
        }
    }
}

pub fn exit_hiding_system(
    mut commands: Commands,
    hiding_screen_query: Query<Entity, With<super::HidingScreenComponent>>,
    keyboard_input: Res<Input<KeyCode>>,
    mut app_state: ResMut<State<GameState>>,
    mut vulnerability_resource: ResMut<VulnerabilityResource>,
) {
    if keyboard_input.just_released(KeyCode::C)
        && app_state.current() == &GameState::PlayerHidingState
    {
        vulnerability_resource.is_hiding = false;
        app_state.pop().unwrap();
        super::despawn_hiding_screen(&mut commands, &hiding_screen_query);
        info!("Exit hiding ")
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
    vulnerability_resource: Res<VulnerabilityResource>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut go_data: ResMut<GameOverData>,
    mut day_resource: ResMut<DayCycleResource>,
    mut pizza_delivery_resource: ResMut<PizzaDeliveryResource>,
) {
    if app_state.current() == &GameState::PlayerSleepingState {
        sleep_resource.sleep_timer.tick(time.delta());

        // wake up player when timer finished
        if sleep_resource.sleep_timer.just_finished() {
            sleep_resource.sleep_timer.reset();

            // if vulnerable, jumpscare and game over, otherwise wake up
            if vulnerability_resource.bool_vulnerabilities[&BoolVulnerabilityType::BedDoorLeftOpen]
                && !vulnerability_resource.enemies.is_empty()
            {
                audio.play(asset_server.load("audio/dramatic_scare.mp3"));
                spawn_npc(
                    "textures/npcs/npc_1_right_spritesheet.png",
                    Vec2::new(-18.0, 57.0),
                    &mut commands,
                    &asset_server,
                    &mut texture_atlases,
                );
                if app_state.current() == &GameState::PlayerSleepingState {
                    go_data.reason = Some(GameOverReason::DoorLeftOpen);
                    go_data.hide_player_sprite = true;
                    app_state.set(GameState::GameOverState).unwrap();
                }
            } else {
                super::despawn_player_in_bed(&mut commands, &player_in_bed_query);
                audio.play(asset_server.load("audio/get_out_bed.mp3"));
                sleepiness.0 = 100.;
                if app_state.current() == &GameState::PlayerSleepingState {
                    app_state.pop().unwrap();
                }
                day_resource.sleep();

                // deliver pizza if ordered
                let pizza_time = pizza_delivery_resource
                    .delivery_timer
                    .duration()
                    .as_secs_f32()
                    - 0.1;

                if let PizzaDeliveryStatus::Ordered = pizza_delivery_resource.status {
                    pizza_delivery_resource
                        .delivery_timer
                        .set_elapsed(Duration::from_secs_f32(pizza_time));
                }
                #[cfg(debug_assertions)]
                info!("Player woke up");
            }
        }
    }
}
