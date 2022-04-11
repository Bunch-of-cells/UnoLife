use rand::prelude::SliceRandom;

pub struct Game {
    pub board: [u8; 16],
    pub moves: u32,
}

impl Game {
    pub fn new() -> Game {
        let mut obj = Game {
            board: [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 0],
            moves: 0,
        };
        obj.shuffle();
        obj
    }

    fn shuffle(&mut self) {
        self.board.shuffle(&mut rand::thread_rng());
        while self.is_over() || !self.is_solvable() {
            self.board.shuffle(&mut rand::thread_rng());
        }
    }

    fn is_solvable(&self) -> bool {
        let blank_index = self.board.iter().position(|x| *x == 0).unwrap();

        let blank_row = 4 - blank_index / 4;
        let inversion_count = self.count_inversion();

        if blank_row % 2 == 0 {
            inversion_count % 2 == 1
        } else {
            inversion_count % 2 == 0
        }
    }

    fn count_inversion(&self) -> u16 {
        let mut count = 0;
        for i in 0..16 {
            for j in 0..16 {
                if i == j {
                    continue;
                }

                if self.board[i] == 0 || self.board[j] == 0 {
                    continue;
                }

                if self.board[i] > self.board[j] && i < j {
                    count += 1;
                }
            }
        }

        count
    }

    pub fn reset(&mut self) {
        self.shuffle();
        self.moves = 0;
    }

    pub fn step(&mut self, step: Direction) {
        let step = step.invert();
        let blank_index = self.board.iter().position(|x| *x == 0).unwrap();

        match step {
            Direction::Up => {
                if blank_index >= 4 {
                    self.board.swap(blank_index, blank_index - 4);
                    self.moves += 1;
                }
            }
            Direction::Down => {
                if blank_index < 12 {
                    self.board.swap(blank_index, blank_index + 4);
                    self.moves += 1;
                }
            }
            Direction::Left => {
                if blank_index % 4 != 0 {
                    self.board.swap(blank_index, blank_index - 1);
                    self.moves += 1;
                }
            }
            Direction::Right => {
                if blank_index % 4 != 3 {
                    self.board.swap(blank_index, blank_index + 1);
                    self.moves += 1;
                }
            }
        }
    }

    pub fn is_over(&self) -> bool {
        self.board == [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 0]
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn invert(&self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}
