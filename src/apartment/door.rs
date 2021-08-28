use crate::{
    apartment::{
        interactable::despawn_interactable_icons, player::PlayerComponent, InteractableComponent,
        InteractableType, InteractablesResource,
    },
    misc::game_over::{GameOverData, GameOverReason},
    vulnerability::{spawn_npc, AtDoorType, VulnerabilityResource},
};

use crate::states::GameState;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

/// Handles opening and closing of door
pub fn interact_door_system(
    mut commands: Commands,
    player_query: Query<&PlayerComponent>,
    interactable_query: Query<(Entity, &InteractableComponent)>,
    interactable_icon_query: Query<Entity, With<super::interactable::InteractableIconComponent>>,
    hallway_cover_query: Query<Entity, With<super::HallwayCoverComponent>>,
    interactables_resource: Res<InteractablesResource>,
    keyboard_input: Res<Input<KeyCode>>,
    mut app_state: ResMut<State<GameState>>,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    vulnerability_resource: Res<VulnerabilityResource>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut go_data: ResMut<GameOverData>,
    audio: Res<Audio>,
) {
    for player_component in player_query.iter() {
        if let Some(InteractableType::ClosedDoor) = player_component.interactable_in_range {
            // open door
            if keyboard_input.just_pressed(KeyCode::E)
                && app_state.current() == &GameState::MainGame
            {
                #[cfg(debug_assertions)]
                info!("Door opened");
                // despawn the closed door
                for (interactable_entity, interactable_component) in interactable_query.iter() {
                    if let InteractableType::ClosedDoor = interactable_component.interactable_type {
                        commands.entity(interactable_entity).despawn();
                        super::interactable::despawn_interactable_icons(
                            &mut commands,
                            &interactable_icon_query,
                        );
                        super::despawn_hallway_cover(&mut commands, &hallway_cover_query);
                        audio.play(asset_server.load("audio/door_opening.mp3"));
                    }
                }

                // spawn an open door
                spawn_open_door(&mut commands, &interactables_resource);

                // check if enemy at door and game over if so
                if let AtDoorType::NPC = vulnerability_resource.at_door {
                    spawn_npc(
                        "textures/npcs/npc_1_forward_spritesheet.png",
                        Vec2::new(-222.0, 164.0),
                        &mut commands,
                        &asset_server,
                        &mut texture_atlases,
                    );
                    if app_state.current() == &GameState::MainGame {
                        go_data.reason = Some(GameOverReason::LetThemIn);
                        app_state.set(GameState::GameOverState).unwrap();
                        #[cfg(debug_assertions)]
                        info!("Console closed");
                    }
                    audio.play(asset_server.load("audio/dramatic_scare.mp3"));
                }
            } else if keyboard_input.just_pressed(KeyCode::C)
                && app_state.current() == &GameState::MainGame
            {
                app_state.push(GameState::PeepholeOpenedState).unwrap();
                match vulnerability_resource.at_door {
                    crate::vulnerability::AtDoorType::None => super::spawn_peephole(
                        "textures/peepholes/peephole_none.png",
                        &mut commands,
                        &asset_server,
                        &mut materials,
                    ),
                    crate::vulnerability::AtDoorType::DeliveryPerson => super::spawn_peephole(
                        "textures/peepholes/peephole_pizza.png",
                        &mut commands,
                        &asset_server,
                        &mut materials,
                    ),
                    crate::vulnerability::AtDoorType::NPC => super::spawn_peephole(
                        "textures/peepholes/peephole_npc_1.png",
                        &mut commands,
                        &asset_server,
                        &mut materials,
                    ),
                }

                despawn_interactable_icons(&mut commands, &interactable_icon_query);
                info!("Checking peephole")
            }
        } else if let Some(InteractableType::OpenDoor) = player_component.interactable_in_range {
            if keyboard_input.just_pressed(KeyCode::E)
                && app_state.current() == &GameState::MainGame
            {
                #[cfg(debug_assertions)]
                info!("Door closed");
                // despawn the open door
                for (interactable_entity, interactable_component) in interactable_query.iter() {
                    if let InteractableType::OpenDoor = interactable_component.interactable_type {
                        commands.entity(interactable_entity).despawn();
                        super::interactable::despawn_interactable_icons(
                            &mut commands,
                            &interactable_icon_query,
                        );
                        super::spawn_hallway_cover(&mut commands, &asset_server, &mut materials);
                        audio.play(asset_server.load("audio/door_shutting.mp3"));
                    }
                }

                // spawn an open door
                spawn_closed_door(&mut commands, &interactables_resource);
            }
        }
    }
}

pub fn exit_peephole_system(
    mut commands: Commands,
    peephole_query: Query<Entity, With<super::PeepholeComponent>>,
    keyboard_input: Res<Input<KeyCode>>,
    mut app_state: ResMut<State<GameState>>,
) {
    if keyboard_input.just_released(KeyCode::C)
        && app_state.current() == &GameState::PeepholeOpenedState
    {
        app_state.pop().unwrap();
        super::despawn_peepholes(&mut commands, &peephole_query);
        info!("Leaving peephole")
    }
}

/// Spawns a closed door (with collider)
pub fn spawn_closed_door(commands: &mut Commands, interactables_resource: &InteractablesResource) {
    // spawn door
    let interactable_type = InteractableType::ClosedDoor;
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
        .insert_bundle(ColliderBundle {
            shape: ColliderShape::cuboid(
                interactable_data.collider_size.x,
                interactable_data.collider_size.y,
            ),
            material: ColliderMaterial {
                friction: 0.0,
                restitution: 0.0,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(RigidBodyPositionSync::Discrete)
        .insert(Name::new("Closed Door"));
}

/// Spawns an open door (no collider)
pub fn spawn_open_door(commands: &mut Commands, interactables_resource: &InteractablesResource) {
    // spawn door
    let interactable_type = InteractableType::OpenDoor;
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
        .insert(Name::new("Open Door"));
}
