use std::{error::Error, fmt::Display};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Board {
    pub cells: [[Mark; 3]; 3],
    pub turn: Mark,
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(u8)]
pub enum Mark {
    X,
    O,
    None,
}

impl Mark {
    pub fn invert(&self) -> Mark {
        match self {
            Mark::X => Mark::O,
            Mark::O => Mark::X,
            Mark::None => Mark::None,
        }
    }
}

impl Board {
    pub fn new() -> Self {
        Board {
            cells: [[Mark::None; 3]; 3],
            turn: Mark::X,
        }
    }

    /// Returns Error if the move is invalid
    pub fn make_move(&mut self, x: usize, y: usize) -> Option<TicTacToeError> {
        if self.is_over() != Mark::None {
            return Some(TicTacToeError::GameOver);
        }
        if self.cells[x][y] != Mark::None {
            return Some(TicTacToeError::Occupied);
        }
        self.cells[x][y] = self.turn;
        self.turn = self.turn.invert();
        None
    }

    pub fn undo_move(&mut self, x: usize, y: usize) {
        // Assumes move is legal
        self.cells[x][y] = Mark::None;
        self.turn = self.turn.invert();
    }

    pub fn reset(&mut self) {
        self.cells = [[Mark::None; 3]; 3];
        self.turn = Mark::X;
    }

    pub fn is_over(&self) -> Mark {
        for row in 0..3 {
            let mut x_count = 0;
            let mut o_count = 0;
            for col in 0..3 {
                let cell = self.cells[row][col];
                match cell {
                    Mark::X => x_count += 1,
                    Mark::O => o_count += 1,
                    Mark::None => (),
                }
            }
            if x_count == 3 {
                return Mark::X;
            } else if o_count == 3 {
                return Mark::O;
            }
        }

        for col in 0..3 {
            let mut x_count = 0;
            let mut o_count = 0;
            for row in 0..3 {
                let cell = self.cells[row][col];
                match cell {
                    Mark::X => x_count += 1,
                    Mark::O => o_count += 1,
                    Mark::None => (),
                }
            }
            if x_count == 3 {
                return Mark::X;
            } else if o_count == 3 {
                return Mark::O;
            }
        }

        {
            let mut x_count = 0;
            let mut o_count = 0;

            for col in 0..3 {
                let cell = self.cells[col][col];
                match cell {
                    Mark::X => x_count += 1,
                    Mark::O => o_count += 1,
                    Mark::None => (),
                }
            }

            if x_count == 3 {
                return Mark::X;
            } else if o_count == 3 {
                return Mark::O;
            }
        }

        {
            let mut x_count = 0;
            let mut o_count = 0;

            for col in 0..3 {
                let cell = self.cells[col][2 - col];
                match cell {
                    Mark::X => x_count += 1,
                    Mark::O => o_count += 1,
                    Mark::None => (),
                }
            }

            if x_count == 3 {
                return Mark::X;
            } else if o_count == 3 {
                return Mark::O;
            }
        }

        Mark::None
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
