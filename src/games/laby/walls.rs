use rand::Rng;

pub const ALL_DIRECTIONS: &str = "
    
\\                   /
 \\                 /
  \\               / 
  |              / 
  |__            |
  | |\\         __|
  | | \\      /|  |
  | |  \\    / |  |
  | |   \\__|  |  |
  | |   |__|  |  |
  | |   /  |  |  |
  | |  /    \\ |  |
  | | /      \\|  |
  |_|/        |__|
  |              |
  |              | 
  /              \\ 
 /                \\ 
/                  \\
";

pub const FRONT: &str = "
    
\\                  /
 \\                /
  \\              / 
   \\            / 
    \\          /   
    |\\        /|
    | \\      / |
    |  \\    /  |
    |   \\__|   |
    |   |__|   |
    |   /  |   |
    |  /    \\  |
    | /      \\ |
    |/        \\|
    /          \\  
   /            \\ 
  /              \\ 
 /                \\ 
/                  \\
";

pub const LEFT_FRONT: &str = "
    
\\                  /
 \\                /
  \\              / 
  |             / 
  |__          / 
  | |\\        /
  | | \\      /  
  | |  \\    /   
  | |   \\__|    
  | |   |__|    
  | |   /  |    
  | |  /    \\   
  | | /      \\  
  |_|/        \\
  |            \\
  |             \\ 
  /              \\ 
 /                \\ 
/                  \\
";

pub const RIGHT_FRONT: &str = "
    
\\                   /
 \\                 /
  \\               / 
   \\             / 
    \\            |
     \\         __|
      \\      /|  |
       \\    / |  |
        \\__|  |  |
        |__|  |  |
        /  |  |  |
       /    \\ |  |
      /      \\|  |
     /        |__|
    /            |
   /             | 
  /              \\ 
 /                \\ 
/                  \\
";

pub const LEFT: &str = "
    
\\                  /
 \\                /
  \\              / 
  |             / 
  |____________/ 
  |            |
  |            | 
  |            | 
  |            | 
  |            | 
  |            | 
  |            | 
  |            | 
  |____________|
  |            \\
  |             \\ 
  /              \\ 
 /                \\ 
/                  \\
";

pub const RIGHT: &str = "
    
\\                   /
 \\                 /
  \\               / 
   \\             / 
    \\            |
     \\___________|
     |           |
     |           |
     |           |
     |           |
     |           |
     |           |
     |           |
     |___________|
    /            |
   /             | 
  /              \\ 
 /                \\ 
/                  \\
";

pub enum Directions {
    All,
    LeftFront,
    Left,
    RightFront,
    Right,
    Front,
}

impl Directions {
    pub fn get_ascii_art(&self) -> &str {
        match &self {
            Directions::All => ALL_DIRECTIONS,
            Directions::LeftFront => LEFT_FRONT,
            Directions::Left => LEFT,
            Directions::RightFront => RIGHT_FRONT,
            Directions::Right => RIGHT,
            Directions::Front => FRONT,
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
}