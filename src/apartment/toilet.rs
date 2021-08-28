use crate::{
    apartment::{
        player::{PeePeePooPoo, PlayerComponent},
        InteractableType,
    },
    misc::ui_text::{TextUIAnimation, TextUIData},
};

use crate::states::GameState;
use bevy::prelude::*;

pub const PEE_TIME: f32 = 3.5;

/// Handles interacting with toilet
pub fn interact_toilet_system(
    player_query: Query<&PlayerComponent>,
    peepeepoopoo: Res<PeePeePooPoo>,
    keyboard_input: Res<Input<KeyCode>>,
    mut app_state: ResMut<State<GameState>>,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
    mut ui_bottom_text: ResMut<TextUIData>,
    windows: Res<Windows>,
    time: Res<Time>,
    mut anim_data: ResMut<TextUIAnimation>,
) {
    for player_component in player_query.iter() {
        if let Some(InteractableType::Toilet) = player_component.interactable_in_range {
            if keyboard_input.just_pressed(KeyCode::E)
                && app_state.current() == &GameState::MainGame
            {
                if peepeepoopoo.0 <= 50. {
                    #[cfg(debug_assertions)]
                    info!("Using toilet");
                    audio.play(asset_server.load("audio/peeing.mp3"));
                    if app_state.current() == &GameState::MainGame {
                        app_state.push(GameState::PlayerPeeingState).unwrap();
                    }
                } else {
                    ui_bottom_text.show_text(
                        &mut anim_data,
                        &windows,
                        &time,
                        "I don't need to pee yet".to_string(),
                    );
                }
            }
        }
    }
}

// Tracks player peeing
pub struct PeeingResource {
    pub pee_timer: Timer,
}

/// Handles player peeing
pub fn peeing_system(
    mut app_state: ResMut<State<GameState>>,
    mut peeing_resource: ResMut<PeeingResource>,
    mut peepeepoopoo: ResMut<PeePeePooPoo>,
    time: Res<Time>,
) {
    if app_state.current() == &GameState::PlayerPeeingState {
        peeing_resource.pee_timer.tick(time.delta());

        // stop peeing when timer finished
        if peeing_resource.pee_timer.just_finished() {
            peeing_resource.pee_timer.reset();

            peepeepoopoo.0 = 100.;
            if app_state.current() == &GameState::PlayerPeeingState {
                app_state.pop().unwrap();
            }
            #[cfg(debug_assertions)]
            info!("Player done peeing");
        }
    }
}
