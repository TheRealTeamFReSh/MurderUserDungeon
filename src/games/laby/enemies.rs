use rand::Rng;

use super::art;

pub enum EnemyType {
    Rat,
    Bat,
}

impl EnemyType {
    pub fn get_ascii_art(&self) -> &str {
        match self {
            EnemyType::Rat => art::RAT,
            EnemyType::Bat => art::BAT,
        }
    }

    pub fn to_display_str(&self) -> &str {
        match self {
            EnemyType::Rat => "rat",
            EnemyType::Bat => "bat",
        }
    }

    pub fn get_random_enemy() -> EnemyType {
        let index = rand::thread_rng().gen_range(0..2);
        match index {
            0 => EnemyType::Rat,
            _ => EnemyType::Bat,
        }
    }
}