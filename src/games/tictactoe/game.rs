use bevy::prelude::*;
use rand::Rng;

use crate::{console::event::PrintConsoleEvent, games::{ConsoleGamesData, GameList}, vulnerability::{BoolVulnerabilityType, VulnerabilityResource}};

#[derive(Default)]
pub struct TicTacToeData {
    pub has_seen_tutorial: bool,
    // 0 = empty, 1 = player, 2 = ai
    pub grid: [[usize; 3]; 3],
    pub current_turn: TurnType,
    pub waiting_for_input: bool,
    pub turn_number: usize,
}

impl TicTacToeData {
    pub fn reset(&mut self) {
        self.has_seen_tutorial = false;
        self.grid = [[0; 3]; 3];
        self.current_turn = TurnType::PlayerTurn;
        self.waiting_for_input = false;
        self.turn_number = 0;
    }

    fn get_position(letter: &str) -> Option<(usize, usize)> {
        match letter.to_lowercase().as_str() {
            "a" => Some((0, 0)),
            "b" => Some((0, 1)),
            "c" => Some((0, 2)),
            "d" => Some((1, 0)),
            "e" => Some((1, 1)),
            "f" => Some((1, 2)),
            "g" => Some((2, 0)),
            "h" => Some((2, 1)),
            "i" => Some((2, 2)),
            _ => None,
        }
    }

    fn get_pawn(&self, i: usize, j: usize) -> &str {
        match self.grid[i][j] {
            0 => " ",
            1 => "X",
            _ => "O",
        }
    }

    fn has_space_available(&self) -> bool {
        for i in 0..3 {
            for j in 0..3 {
                if self.grid[i as usize][j as usize] == 0 {
                    return true;
                }   
            }
        }
        false
    }

    fn has_winner(&self) -> usize {
        for k in 0..3 {
            if self.grid[k][0] == self.grid[k][1] && self.grid[k][1] == self.grid[k][2] && self.grid[k][0] != 0 { return self.grid[k][0]; }
            if self.grid[0][k] == self.grid[1][k] && self.grid[1][k] == self.grid[2][k] && self.grid[0][k] != 0 { return self.grid[0][k]; }
        }
        if ((self.grid[0][0] == self.grid[1][1] && self.grid[1][1] == self.grid[2][2]) ||
            (self.grid[2][0] == self.grid[1][1] && self.grid[1][1] == self.grid[0][2])) && self.grid[1][1] != 0 { return self.grid[1][1]; }

        0
    }
}

pub enum TurnType {
    PlayerTurn,
    AITurn,    
}
impl Default for TurnType {
    fn default() -> Self {
        TurnType::PlayerTurn
    }
}

pub fn display_tutorial() -> String {
    let mut res = String::from("\n\n\nTicTactToe Tutorial\n");
    res.push_str("-------------------\n");

    res.push_str("
The goal is to align 3 'X' the computer will try
to align 3 'O'. You have to beat the computer!
    
The grid is made of 9 cells from 'A' to 'I'.

   A | B | C
   _________
   D | E | F
   _________
   G | H | I
    
To play, type: 'place A' for example...
    
Good luck!\n");

    res
}

pub fn game_loop(
    mut cg_data: ResMut<ConsoleGamesData>,
    mut ttt_data: ResMut<TicTacToeData>,
    mut console_writer: EventWriter<PrintConsoleEvent>,
    mut vuln_res: ResMut<VulnerabilityResource>,
) {
    if !ttt_data.has_seen_tutorial {
        ttt_data.turn_number = 1;
        console_writer.send(PrintConsoleEvent(display_tutorial()));
        ttt_data.has_seen_tutorial = true;
    }

    if !ttt_data.waiting_for_input {

        console_writer.send(PrintConsoleEvent(format!("---------------------------\nTurn number: {}", ttt_data.turn_number)));
        console_writer.send(PrintConsoleEvent(display_grid(&ttt_data)));

        // check if there is a winner
        match ttt_data.has_winner() {
            1 => {
                console_writer.send(PrintConsoleEvent("CONGRATS!!! You won".to_string()));
                cg_data.loaded_game = GameList::None;
                ttt_data.reset();
                return;
            }
            2 => {
                console_writer.send(PrintConsoleEvent("You lost like a *****".to_string()));
                cg_data.loaded_game = GameList::None;
                ttt_data.reset();
                *vuln_res
                    .bool_vulnerabilities
                    .get_mut(&BoolVulnerabilityType::TicTacToeLosing)
                    .unwrap() = true;
                return;
            }
            _ => ()
        }

        // check if pawns can be placed
        if !ttt_data.has_space_available() {
            console_writer.send(PrintConsoleEvent("It's a tie... What are you doing ??!".to_string()));
            cg_data.loaded_game = GameList::None;
            ttt_data.reset();
            return;
        }

        match ttt_data.current_turn {
            TurnType::PlayerTurn => {
                ttt_data.turn_number += 1;
                console_writer.send(PrintConsoleEvent("It's your turn to play [A-I]:".to_string()));
            }

            TurnType::AITurn => {
                // pick a valid position
                loop {
                    let index = rand::thread_rng().gen_range(0..9);
                    let letter = ["a", "b", "c", "d", "e", "f", "g", "h", "i"][index];

                    let (i, j) = TicTacToeData::get_position(letter).unwrap();
                    if ttt_data.grid[i][j] == 0 {
                        ttt_data.grid[i][j] = 2;
                        break;
                    }
                }


                ttt_data.current_turn = TurnType::PlayerTurn;
                ttt_data.waiting_for_input = false;
                return;
            }
        }
        ttt_data.waiting_for_input = true;
    }
}

pub fn play_position(
    place: &str,
    ttt_data: &mut ResMut<TicTacToeData>,
    console_writer: &mut EventWriter<PrintConsoleEvent>
) {
    if ttt_data.waiting_for_input {
        if let Some(position) = TicTacToeData::get_position(place) {
            let (i, j) = position;
            match ttt_data.grid[i][j] {
                1 => console_writer.send(PrintConsoleEvent("You already placed a pawn here".to_string())),
                2 => console_writer.send(PrintConsoleEvent("You can't place a pawn on top of your opponent's".to_string())),
                _ => {
                    console_writer.send(PrintConsoleEvent(format!("You place a pawn at position ({}, {})", i, j)));
                    ttt_data.grid[i][j] = 1;
                    ttt_data.current_turn = TurnType::AITurn;
                    ttt_data.waiting_for_input = false;
                }
            }
        } else {
            console_writer.send(PrintConsoleEvent(format!("'{}' is not a valid position (valid: [A-I])", place)));
        }
    }
}

pub fn display_grid(
    ttt_data: &ResMut<TicTacToeData>,
) -> String {
    let mut res = String::from("Current grid | Positions : \n");

    res.push_str(&format!(
        "{} | {} | {}        A | B | C \n",
        ttt_data.get_pawn(0, 0),
        ttt_data.get_pawn(0, 1),
        ttt_data.get_pawn(0, 2),
    ));
    res.push_str("_________       _________\n");
    res.push_str(&format!(
        "{} | {} | {}        D | E | F \n",
        ttt_data.get_pawn(1, 0),
        ttt_data.get_pawn(1, 1),
        ttt_data.get_pawn(1, 2),
    ));
    res.push_str("_________       _________\n");
    res.push_str(&format!(
        "{} | {} | {}        G | H | I\n\n",
        ttt_data.get_pawn(2, 0),
        ttt_data.get_pawn(2, 1),
        ttt_data.get_pawn(2, 2),
    ));

    res
}