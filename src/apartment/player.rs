use crate::apartment::PLAYER_Z;
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
        .insert(PlayerComponent { speed: 1.5 })
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
        .insert(RigidBodyPositionSync::Discrete)
        .insert(Name::new("Player"))
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
) {
    for (player, mut rb_vels) in player_info.iter_mut() {
        // get key presses
        let up = keyboard_input.pressed(KeyCode::W);
        let down = keyboard_input.pressed(KeyCode::S);
        let left = keyboard_input.pressed(KeyCode::A);
        let right = keyboard_input.pressed(KeyCode::D);

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
