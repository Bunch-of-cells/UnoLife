use std::{error::Error, fmt::Display};

mod ai;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Board {
    cells: [[Option<Mark>; 3]; 3],
    turn: Mark,
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(u8)]
pub enum Mark {
    X,
    O,
}

impl Board {
    pub fn new() -> Self {
        Board {
            cells: [[None; 3]; 3],
            turn: Mark::X,
        }
    }

    pub fn mov(&mut self, x: usize, y: usize) -> Result<Option<Mark>, TicTacToeError> {
        if self.is_over().is_some() {
            return Err(TicTacToeError::GameOver);
        }
        if self.cells[x][y].is_some() {
            return Err(TicTacToeError::Occupied);
        }
        self.cells[x][y] = Some(self.turn);
        self.turn = match self.turn {
            Mark::X => Mark::O,
            Mark::O => Mark::X,
        };
        Ok(self.is_over())
    }

    pub fn is_over(&self) -> Option<Mark> {
        let mut x_count = 0;
        let mut o_count = 0;
        for row in &self.cells {
            for cell in row {
                match cell {
                    Some(Mark::X) => x_count += 1,
                    Some(Mark::O) => o_count += 1,
                    None => (),
                }
            }
        }
        if x_count == 3 {
            Some(Mark::X)
        } else if o_count == 3 {
            Some(Mark::O)
        } else {
            None
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn board_works() {
        let mut board = Board::new();
        assert_eq!(board.mov(0, 0), Ok(None));
        assert_eq!(board.mov(0, 0), Err(TicTacToeError::Occupied));
        assert_eq!(board.mov(1, 0), Ok(None));
        assert_eq!(board.mov(0, 1), Ok(None));
        assert_eq!(board.mov(2, 0), Ok(None));
        assert_eq!(board.mov(0, 2), Ok(Some(Mark::X)));
        assert_eq!(board.mov(1, 1), Err(TicTacToeError::GameOver));
        assert_eq!(board.is_over(), Some(Mark::X));
    }
}
