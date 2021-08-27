use crate::{
    apartment::{
        player::{Hunger, PlayerComponent},
        InteractableComponent, InteractableType, InteractablesResource,
    },
    vulnerability::{AtDoorType, VulnerabilityResource},
};

use crate::states::GameState;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub const CALL_TIME: f32 = 5.0;
pub const EAT_TIME: f32 = 5.4;
pub const DELIVERY_TIME: f32 = 1.0; // in in-game hours (uses day_cycle)

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
) {
    for player_component in player_query.iter() {
        if let Some(InteractableType::Pizza) = player_component.interactable_in_range {
            if keyboard_input.just_pressed(KeyCode::E)
                && app_state.current() == &GameState::MainGame
            {
                if let PizzaDeliveryStatus::Delivered = pizza_delivery_resource.status {
                    if hunger.0 <= 50. {
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
                        info!("Not hungry enough to eat.")
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
) {
    for player_component in player_query.iter() {
        if let Some(InteractableType::Phone) = player_component.interactable_in_range {
            if keyboard_input.just_pressed(KeyCode::E)
                && app_state.current() == &GameState::MainGame
            {
                match pizza_delivery_resource.status {
                    PizzaDeliveryStatus::Delivered => info!("Pizza is already here!"),
                    PizzaDeliveryStatus::Ordered => info!("I already ordered pizza!"),
                    PizzaDeliveryStatus::AtDoor => info!("I already ordered pizza!"),
                    PizzaDeliveryStatus::Unordered => {
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
) {
    match pizza_delivery_resource.status {
        PizzaDeliveryStatus::Ordered => {
            pizza_delivery_resource.delivery_timer.tick(time.delta());

            if pizza_delivery_resource.delivery_timer.just_finished() {
                pizza_delivery_resource.status = PizzaDeliveryStatus::AtDoor;
                vulnerability_resource.at_door = AtDoorType::DeliveryPerson;
                audio.play(asset_server.load("audio/knocking.mp3"));
                info!("Pizza is here!");
            }
        }
        PizzaDeliveryStatus::AtDoor => {
            for interactable_component in interactable_query.iter() {
                if let InteractableType::OpenDoor = interactable_component.interactable_type {
                    pizza_delivery_resource.status = PizzaDeliveryStatus::Delivered;
                    super::spawn_pizza(&mut commands, &asset_server, &mut materials);
                    spawn_pizza_interactable(&mut commands, &interactables_resource);
                    vulnerability_resource.at_door = AtDoorType::None;
                    info!("I have the pizza.")
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
