use rand::{
    distributions::{Bernoulli, Distribution},
    Rng,
};

#[derive(Debug, Clone, PartialEq)]
pub struct Game {
    pub board: [u32; Self::GRIDSIZE],
    pub score: u32,
    pub state: GameState,
}

impl Game {
    pub const LENGTH: usize = 4;
    pub const WIDTH: usize = 4;
    pub const GRIDSIZE: usize = Self::LENGTH * Self::WIDTH;

    pub fn new() -> Self {
        let mut obj = Self {
            board: [0; Self::GRIDSIZE],
            score: 0,
            state: GameState::Playing,
        };
        obj.add_random_tile();
        obj.add_random_tile();
        obj
    }

    fn slide(&mut self, direction: Direction) {
        self.score += match direction {
            Direction::Up => self.slide_up(),
            Direction::Down => self.slide_down(),
            Direction::Left => self.slide_left(),
            Direction::Right => self.slide_right(),
        };
    }

    fn slide_up(&mut self) -> u32 {
        let mut score = 0;
        for column in 0..Self::LENGTH {
            for row in 0..Self::WIDTH {
                let index = row * Self::WIDTH + column;
                let mut value = self.board[index];
                let mut empty_index = None;
                for row_offset in 0..row {
                    let other_index = (row - row_offset - 1) * Self::WIDTH + column;
                    if self.board[other_index] == 0 {
                        empty_index = Some(other_index);
                    } else if self.board[other_index] == value {
                        value *= 2;
                        score += value;
                        empty_index = Some(other_index);
                    } else {
                        break;
                    }
                }
                if let Some(empty_index) = empty_index {
                    self.board[empty_index] = value;
                    self.board[index] = 0;
                }
            }
        }
        score
    }

    fn slide_down(&mut self) -> u32 {
        let mut score = 0;
        for column in 0..Self::LENGTH {
            for row in (0..Self::WIDTH).rev() {
                let index = row * Self::WIDTH + column;
                let mut value = self.board[index];
                let mut empty_index = None;
                for row_offset in 0..(Self::WIDTH - row - 1) {
                    let other_index = (row + row_offset + 1) * Self::WIDTH + column;
                    if self.board[other_index] == 0 {
                        empty_index = Some(other_index);
                    } else if self.board[other_index] == value {
                        value *= 2;
                        score += value;
                        empty_index = Some(other_index);
                    } else {
                        break;
                    }
                }
                if let Some(empty_index) = empty_index {
                    self.board[empty_index] = value;
                    self.board[index] = 0;
                }
            }
        }
        score
    }

    fn slide_left(&mut self) -> u32 {
        let mut score = 0;
        for row in 0..Self::WIDTH {
            for column in 0..Self::LENGTH {
                let index = row * Self::WIDTH + column;
                let mut value = self.board[index];
                let mut empty_index = None;
                for column_offset in 0..column {
                    let other_index = row * Self::WIDTH + column - column_offset - 1;
                    if self.board[other_index] == 0 {
                        empty_index = Some(other_index);
                    } else if self.board[other_index] == value {
                        value *= 2;
                        score += value;
                        empty_index = Some(other_index);
                    } else {
                        break;
                    }
                }
                if let Some(empty_index) = empty_index {
                    self.board[empty_index] = value;
                    self.board[index] = 0;
                }
            }
        }
        score
    }

    fn slide_right(&mut self) -> u32 {
        let mut score = 0;
        for row in 0..Self::WIDTH {
            for column in (0..Self::LENGTH).rev() {
                let index = row * Self::WIDTH + column;
                let mut value = self.board[index];
                let mut empty_index = None;
                for column_offset in 0..(Self::LENGTH - column - 1) {
                    let other_index = row * Self::WIDTH + column + column_offset + 1;
                    if self.board[other_index] == 0 {
                        empty_index = Some(other_index);
                    } else if self.board[other_index] == value {
                        value *= 2;
                        score += value;
                        empty_index = Some(other_index);
                    } else {
                        break;
                    }
                }
                if let Some(empty_index) = empty_index {
                    self.board[empty_index] = value;
                    self.board[index] = 0;
                }
            }
        }
        score
    }

    fn add_random_tile(&mut self) {
        let mut rng = rand::thread_rng();
        let mut empty_indices = Vec::new();
        for index in 0..Self::GRIDSIZE {
            if self.board[index] == 0 {
                empty_indices.push(index);
            }
        }
        if empty_indices.is_empty() {
            return;
        }
        let index = empty_indices[rng.gen_range(0..empty_indices.len())];
        let mut rng = rand::thread_rng();
        let bernoulli = Bernoulli::new(0.9).unwrap();
        self.board[index] = if bernoulli.sample(&mut rng) { 2 } else { 4 };
    }

    fn set_state(&mut self) {
        if self.board.iter().any(|&v| v == 2048) {
            self.state = GameState::Won;
        } else if self.board.iter().any(|&v| v == 0)
            || (0..3).any(|i| {
                (0..3).any(|j| {
                    self.board[i * Self::WIDTH + j] == self.board[(i + 1) * Self::WIDTH + j]
                        || self.board[i * Self::WIDTH + j] == self.board[i * Self::WIDTH + j + 1]
                } || self.board[i * Self::WIDTH + 3] == self.board[(i + 1) * Self::WIDTH + 3]
             || self.board[3 * Self::WIDTH + i] == self.board[3 * Self::WIDTH + i + 1])
            })
        {
            self.state = GameState::Playing;
        } else {
            self.state = GameState::Lost;
        }
    }

    pub fn reset(&mut self) {
        self.board = [0; Self::GRIDSIZE];
        self.add_random_tile();
        self.add_random_tile();
        self.state = GameState::Playing;
    }

    pub fn step(&mut self, direction: Direction) {
        if self.state != GameState::Playing {
            return;
        }
        self.slide(direction);
        self.add_random_tile();
        self.set_state();
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GameState {
    Won,
    Lost,
    Playing,
}
