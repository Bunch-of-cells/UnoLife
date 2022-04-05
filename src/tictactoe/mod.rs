use std::{error::Error, fmt::Display};

pub mod ai;
pub use ai::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Board {
    pub cells: [[Option<Mark>; 3]; 3],
    pub turn: Mark,
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(u8)]
pub enum Mark {
    X,
    O,
    None,
}

impl Board {
    pub fn new() -> Self {
        Board {
            cells: [[None; 3]; 3],
            turn: Mark::X,
        }
    }

    pub fn mov(&mut self, x: usize, y: usize) -> Result<Mark, TicTacToeError> {
        if self.is_over() != Mark::None {
            return Err(TicTacToeError::GameOver);
        }
        if self.cells[x][y].is_some() {
            return Err(TicTacToeError::Occupied);
        }
        self.cells[x][y] = Some(self.turn);
        self.turn = match self.turn {
            Mark::X => Mark::O,
            Mark::O => Mark::X,
            Mark::None => Mark::None,
        };
        Ok(self.is_over())
    }

    pub fn is_over(&self) -> Mark {
        let mut x_count = 0;
        let mut o_count = 0;
        for row in &self.cells {
            for cell in row {
                match cell {
                    Some(Mark::X) => x_count += 1,
                    Some(Mark::O) => o_count += 1,
                    None | Some(Mark::None) => (),
                }
            }
        }
        if x_count == 3 {
            Mark::X
        } else if o_count == 3 {
            Mark::O
        } else {
            Mark::None
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TicTacToeError {
    Occupied,
    GameOver,
}

impl Error for TicTacToeError {}
impl Display for TicTacToeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TicTacToeError::Occupied => write!(f, "Cell is occupied"),
            TicTacToeError::GameOver => write!(f, "Game is over"),
        }
    }
}