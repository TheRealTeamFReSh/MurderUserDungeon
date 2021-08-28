use crate::misc::ui_text::BottomTextUI;
use crate::states::GameState;
use crate::{
    apartment::{
        player::{Hunger, PlayerComponent},
        InteractableComponent, InteractableType, InteractablesResource,
    },
    misc::{
        day_cycle::DAY_LENGTH,
    },
    vulnerability::{AtDoorType, VulnerabilityResource},
};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::Rng;
use std::time::Duration;

pub const CALL_TIME: f32 = 5.0;
pub const EAT_TIME: f32 = 5.4;
pub const DELIVERY_TIME: f32 = 3.0; // in in-game hours (uses day_cycle)
pub const AT_DOOR_TIME: f32 = 0.75; // in in-game hours

// tracks if player has pizza available

pub enum PizzaDeliveryStatus {
    Delivered,
    AtDoor,
    Unordered,
    Ordered,
}
pub struct PizzaDeliveryResource {
    pub status: PizzaDeliveryStatus,
    pub delivery_timer: Timer,
    pub at_door_timer: Timer,
}

/// Handles interacting with bed
pub fn interact_pizza_system(
    mut commands: Commands,
    interactable_query: Query<(Entity, &InteractableComponent)>,
    interactable_icon_query: Query<Entity, With<super::interactable::InteractableIconComponent>>,
    player_query: Query<&PlayerComponent>,
    keyboard_input: Res<Input<KeyCode>>,
    mut app_state: ResMut<State<GameState>>,
    asset_server: Res<AssetServer>,
    hunger: Res<Hunger>,
    pizza_delivery_resource: Res<PizzaDeliveryResource>,
    audio: Res<Audio>,
    mut ui_bundle: ResMut<BottomTextUI>,
) {
    for player_component in player_query.iter() {
        if let Some(InteractableType::Pizza) = player_component.interactable_in_range {
            if keyboard_input.just_pressed(KeyCode::E)
                && app_state.current() == &GameState::MainGame
            {
                if let PizzaDeliveryStatus::Delivered = pizza_delivery_resource.status {
                    if hunger.0 <= 50. {
                        #[cfg(debug_assertions)]
                        info!("Eating pizza");
                        // despawn pizza interactable
                        for (interactable_entity, interactable_component) in
                            interactable_query.iter()
                        {
                            if let InteractableType::Pizza =
                                interactable_component.interactable_type
                            {
                                commands.entity(interactable_entity).despawn();
                                super::interactable::despawn_interactable_icons(
                                    &mut commands,
                                    &interactable_icon_query,
                                );
                            }
                        }
                        audio.play(asset_server.load("audio/eating.mp3"));
                        if app_state.current() == &GameState::MainGame {
                            app_state.push(GameState::PlayerEatingState).unwrap();
                        }
                    } else {
                        ui_bundle.show_text("I'm not hungry yet".to_string());
                    }
                }
            }
        }
    }
}

pub struct EatingResource {
    pub eating_timer: Timer,
}

/// Handles player peeing
pub fn eating_system(
    mut commands: Commands,
    pizza_query: Query<Entity, With<super::PizzaComponent>>,
    mut app_state: ResMut<State<GameState>>,
    mut eating_resource: ResMut<EatingResource>,
    mut hunger: ResMut<Hunger>,
    mut pizza_delivery_resource: ResMut<PizzaDeliveryResource>,
    time: Res<Time>,
) {
    if app_state.current() == &GameState::PlayerEatingState {
        eating_resource.eating_timer.tick(time.delta());

        // stop peeing when timer finished
        if eating_resource.eating_timer.just_finished() {
            eating_resource.eating_timer.reset();

            hunger.0 = 100.;
            if app_state.current() == &GameState::PlayerEatingState {
                app_state.pop().unwrap();
            }
            super::despawn_pizza(&mut commands, &pizza_query);
            pizza_delivery_resource.status = PizzaDeliveryStatus::Unordered;
            #[cfg(debug_assertions)]
            info!("Player done eating");
        }
    }
}

/// Handles interacting with bed
pub fn interact_phone_system(
    player_query: Query<&PlayerComponent>,
    keyboard_input: Res<Input<KeyCode>>,
    mut app_state: ResMut<State<GameState>>,
    asset_server: Res<AssetServer>,
    pizza_delivery_resource: Res<PizzaDeliveryResource>,
    audio: Res<Audio>,
    mut ui_bundle: ResMut<BottomTextUI>,
) {
    for player_component in player_query.iter() {
        if let Some(InteractableType::Phone) = player_component.interactable_in_range {
            if keyboard_input.just_pressed(KeyCode::E)
                && app_state.current() == &GameState::MainGame
            {
                match pizza_delivery_resource.status {
                    PizzaDeliveryStatus::Delivered => ui_bundle.show_text("Pizza is already here".to_string()),
                    PizzaDeliveryStatus::Ordered => ui_bundle.show_text("I already ordered pizza!".to_string()),
                    PizzaDeliveryStatus::AtDoor => ui_bundle.show_text("I already ordered pizza!".to_string()),
                    PizzaDeliveryStatus::Unordered => {
                        #[cfg(debug_assertions)]
                        info!("Using Phone");
                        audio.play(asset_server.load("audio/pizza_orders/pizza_order_1.mp3"));
                        if app_state.current() == &GameState::MainGame {
                            app_state.push(GameState::PlayerOrderingPizzaState).unwrap();
                        }
                    }
                }
            }
        }
    }
}

pub struct OrderingPizzaResource {
    pub call_timer: Timer,
}

pub fn ordering_pizza_system(
    mut app_state: ResMut<State<GameState>>,
    mut order_resource: ResMut<OrderingPizzaResource>,
    mut pizza_delivery_resource: ResMut<PizzaDeliveryResource>,
    time: Res<Time>,
) {
    if app_state.current() == &GameState::PlayerOrderingPizzaState {
        order_resource.call_timer.tick(time.delta());

        // stop peeing when timer finished
        if order_resource.call_timer.just_finished() {
            order_resource.call_timer.reset();

            if app_state.current() == &GameState::PlayerOrderingPizzaState {
                app_state.pop().unwrap();
            }
            pizza_delivery_resource.status = PizzaDeliveryStatus::Ordered;
            #[cfg(debug_assertions)]
            info!("Player done ordering pizza");
        }
    }
}

pub fn pizza_delivery_system(
    mut commands: Commands,
    mut pizza_delivery_resource: ResMut<PizzaDeliveryResource>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    interactables_resource: Res<InteractablesResource>,
    interactable_query: Query<&InteractableComponent>,
    mut vulnerability_resource: ResMut<VulnerabilityResource>,
    time: Res<Time>,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
    mut ui_bundle: ResMut<BottomTextUI>,
) {
    match pizza_delivery_resource.status {
        PizzaDeliveryStatus::Ordered => {
            pizza_delivery_resource.delivery_timer.tick(time.delta());

            if pizza_delivery_resource.delivery_timer.just_finished() {
                ui_bundle.show_text("Pizza is here!".to_string());

                // random chance of spawning npc instead based on vulnerability level
                if rand::thread_rng().gen::<f32>() * vulnerability_resource.vulnerability_factor
                    > 0.1
                {
                    vulnerability_resource.at_door = AtDoorType::Npc;
                    audio.play(asset_server.load("audio/knocking.mp3"));
                    pizza_delivery_resource
                        .delivery_timer
                        .set_elapsed(Duration::from_secs_f32(
                            (2.0 / 5.0) * DAY_LENGTH * (DELIVERY_TIME) / 24.0,
                        ));
                } else {
                    pizza_delivery_resource.status = PizzaDeliveryStatus::AtDoor;
                    vulnerability_resource.at_door = AtDoorType::DeliveryPerson;
                    audio.play(asset_server.load("audio/knocking.mp3"));
                }
            }
        }
        PizzaDeliveryStatus::AtDoor => {
            // countdown time at door and leave if player hasn't answered
            pizza_delivery_resource.at_door_timer.tick(time.delta());

            if pizza_delivery_resource.at_door_timer.just_finished() {
                // deliveryperson leaves
                pizza_delivery_resource.status = PizzaDeliveryStatus::Unordered;
                vulnerability_resource.at_door = AtDoorType::None;
                ui_bundle.show_text("Delivery person left :(!".to_string());
            } else {
                for interactable_component in interactable_query.iter() {
                    if let InteractableType::OpenDoor = interactable_component.interactable_type {
                        pizza_delivery_resource.status = PizzaDeliveryStatus::Delivered;
                        super::spawn_pizza(&mut commands, &asset_server, &mut materials);
                        spawn_pizza_interactable(&mut commands, &interactables_resource);
                        vulnerability_resource.at_door = AtDoorType::None;
                        pizza_delivery_resource.at_door_timer.reset();
                        #[cfg(debug_assertions)]
                        info!("I have the pizza.")
                    }
                }
            }
        }
        _ => {}
    }
}

/// Spawns an open door (no collider)
pub fn spawn_pizza_interactable(
    commands: &mut Commands,
    interactables_resource: &InteractablesResource,
) {
    // spawn door
    let interactable_type = InteractableType::Pizza;
    let interactable_data = &interactables_resource.interactables[&interactable_type];
    commands
        .spawn()
        .insert(InteractableComponent {
            interactable_type,
            range: interactable_data.range,
        })
        .insert_bundle(RigidBodyBundle {
            body_type: RigidBodyType::Static,
            position: interactable_data.position.into(),
            ..Default::default()
        })
        .insert(RigidBodyPositionSync::Discrete)
        .insert(Name::new("Pizza Interactable"));
}
