use rand::Rng;

use super::art;

pub enum ItemType {
    Chest,
    Key,
    Sword,
}

impl ItemType {
    pub fn get_ascii_art(&self) -> &str {
        match self {
            ItemType::Chest => art::CHEST,
            ItemType::Key => art::KEY,
            ItemType::Sword => art::SWORD,
        }
    }

    pub fn _to_display_str(&self) -> &str {
        match self {
            ItemType::Chest => "chest",
            ItemType::Key => "key",
            ItemType::Sword => "sword",
        }
    }

    pub fn get_random_item() -> ItemType {
        let index = rand::thread_rng().gen_range(0..2);
        match index {
            0 => ItemType::Chest,
            1 => ItemType::Key,
            _ => ItemType::Sword,
        }
    }
}