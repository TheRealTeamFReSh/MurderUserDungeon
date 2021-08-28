use bevy::prelude::*;
use rand::seq::SliceRandom;
use ron::de::from_bytes;
use serde::Deserialize;
use std::collections::HashMap;

pub const NPC_COUNT: usize = 3;
pub struct NPCsPlugin;

impl Plugin for NPCsPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.insert_resource(
            from_bytes::<UsernamesResource>(include_bytes!("../../data/usernames.ron")).unwrap(),
        )
        .insert_resource(NPCsResource {
            npcs: HashMap::new(),
        })
        .add_startup_system(generate_npcs_system.system());
    }
}

pub fn generate_npcs_system(
    mut npcs_resource: ResMut<NPCsResource>,
    usernames_resource: Res<UsernamesResource>,
) {
    let mut usernames = usernames_resource.usernames.clone();

    for i in 0..NPC_COUNT {
        let username = usernames.choose(&mut rand::thread_rng()).unwrap().clone();
        usernames.retain(|x| x != &username);

        npcs_resource.npcs.insert(
            username.to_string(),
            NPCData {
                sprite_id: i,
                username: username.to_string(),
            },
        );
    }

    println!("{:?}", npcs_resource.npcs);
}

#[derive(Deserialize)]
pub struct NPCsResource {
    pub npcs: HashMap<String, NPCData>,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct NPCData {
    pub sprite_id: usize,
    pub username: String,
}

#[derive(Deserialize)]
pub struct UsernamesResource {
    pub usernames: Vec<String>,
}
