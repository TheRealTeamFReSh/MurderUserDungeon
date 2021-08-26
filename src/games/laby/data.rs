use rand::Rng;
use serde::Deserialize;

use super::{art, enemies::Enemy, items::ItemType};

#[derive(PartialEq)]
pub enum GameState {
    Tutorial,
    Exploring,
}

pub struct PlayerStats {
    pub health: f32,
    pub max_health: f32,
    pub level: usize,
    pub exp: usize,
    pub damages: f32,
}

impl Default for PlayerStats {
    fn default() -> Self {
        PlayerStats {
            health: 10.0,
            max_health: 10.0,
            level: 1,
            exp: 0,
            damages: 1.0,
        }
    }
}

#[derive(PartialEq)]
pub enum RoomType {
    Corridor,
    Enemy,
    Item,
}

pub struct LabyrinthData {
    pub steps_number: usize,
    pub room_type: RoomType,
    pub enemy: Enemy,
    pub item_type: ItemType,
    pub next_directions: Directions,
    pub has_shown_turn_infos: bool,
    pub wait_for_continue: bool,
    pub game_state: GameState,
    pub description: String,
    pub status_message: String,
}

impl Default for LabyrinthData {
    fn default() -> Self {
        LabyrinthData {
            next_directions: Directions::All,
            game_state: GameState::Tutorial,
            has_shown_turn_infos: false,
            steps_number: 0,
            room_type: RoomType::Corridor,
            wait_for_continue: false,
            description: String::from(""),
            enemy: Enemy::default(),
            item_type: ItemType::Chest,
            status_message: String::from(""),
        }
    }
}

impl LabyrinthData {
    pub fn reset(&mut self) {
        self.steps_number = 0;
        self.next_directions = Directions::All;
        self.has_shown_turn_infos = false;
        self.wait_for_continue = false;
        self.game_state = GameState::Tutorial;
    }
}

// Stores data about the labyrinth
#[derive(Debug, Deserialize)]
pub struct LabyrinthResourceFile {
    pub descriptions: Vec<String>,
    pub tutorial: String,
    pub enemies: Vec<Enemy>, 
}

#[derive(PartialEq)]
pub enum Directions {
    All,
    LeftFront,
    Left,
    RightFront,
    Right,
    Front,
}

pub enum Movement {
    Forward,
    Left,
    Right,
}

impl Movement {
    pub fn from_string(dir: &str) -> Option<Movement> {
        match dir.to_lowercase().as_str() {
            "forward" => Some(Movement::Forward),
            "left" => Some(Movement::Left),
            "right" => Some(Movement::Right),
            _ => None,
        }
    }
}

impl Directions {
    pub fn get_ascii_art(&self) -> &str {
        match &self {
            Directions::All => art::ALL_DIRECTIONS,
            Directions::LeftFront => art::LEFT_FRONT,
            Directions::Left => art::LEFT,
            Directions::RightFront => art::RIGHT_FRONT,
            Directions::Right => art::RIGHT,
            Directions::Front => art::FRONT,
        }
    }

    pub fn get_random_direction() -> Directions {
        let index = rand::thread_rng().gen_range(0..6);
        match index {
            0 => Directions::All,
            1 => Directions::LeftFront,
            2 => Directions::Left,
            3 => Directions::RightFront,
            4 => Directions::Right,
            _ => Directions::Front,
        }
    }

    pub fn to_display(&self) -> String {
        match self {
            Directions::All => "Forward, Left, Right".to_string(),
            Directions::LeftFront => "Forward, Left".to_string(),
            Directions::Left => "Left".to_string(),
            Directions::RightFront => "Forward, Right".to_string(),
            Directions::Right => "Right".to_string(),
            Directions::Front => "Forward".to_string(),
        }
    }

    pub fn can_go_direction(&self, mov: Movement) -> bool {
        if self == &Directions::All { return true; }

        match mov {
            Movement::Forward => {
                if self == &Directions::LeftFront || self == &Directions::RightFront || self == &Directions::Front {
                    return true;
                }
            }

            Movement::Left => {
                if self == &Directions::LeftFront || self == &Directions::Left {
                    return true;
                }
            }

            Movement::Right => {
                if self == &Directions::RightFront || self == &Directions::Right {
                    return true;
                }
            }
        }

        false
    }
}