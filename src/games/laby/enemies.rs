use rand::Rng;
use serde::Deserialize;

use super::art;

#[derive(Debug, Deserialize, Clone)]
pub struct Enemy {
    pub kind: EnemyType,
    pub description: String,
    pub health: f32,
    pub max_health: f32,
    pub exp: usize,
    pub damages: f32,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub enum EnemyType {
    Rat,
    Bat,
    Boss,
}

impl EnemyType {
    pub fn _to_display_str(&self) -> &str {
        match self {
            EnemyType::Rat => "rat",
            EnemyType::Bat => "bat",
            EnemyType::Boss => "boss",
        }
    }
}

impl Enemy {
    pub fn get_ascii_art(&self) -> &str {
        match self.kind {
            EnemyType::Rat => art::RAT,
            EnemyType::Bat => art::BAT,
            EnemyType::Boss => art::BOSS,
        }
    }

    pub fn get_random_enemy(enemies: &[Enemy]) -> &Enemy {
        let index = rand::thread_rng().gen_range(0..enemies.len());
        return enemies.get(index).unwrap();
    }
}

impl Default for Enemy {
    fn default() -> Self {
        Enemy {
            kind: EnemyType::Rat,
            description: "Rat description".to_string(),
            max_health: 3.0,
            health: 3.0,
            exp: 5,
            damages: 0.5,
        }
    }
}
