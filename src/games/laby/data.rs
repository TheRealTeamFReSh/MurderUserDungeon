use super::walls::Directions;

pub struct LabyrinthData {
    pub steps_number: usize,
    pub next_directions: Directions,
}

impl Default for LabyrinthData {
    fn default() -> Self {
        LabyrinthData {
            steps_number: 0,
            next_directions: Directions::All,
        }
    }
}