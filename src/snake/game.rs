use rand::prelude::SliceRandom;

pub struct Snake {
    pub body: Vec<SnakeCell>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SnakeCell {
    pub x: u32,
    pub y: u32,
    pub dir: Option<Direction>,
}

impl SnakeCell {
    fn new(x: u32, y: u32) -> Self {
        Self { x, y, dir: None }
    }

    fn into(self) -> FoodCell {
        FoodCell {
            x: self.x,
            y: self.y,
        }
    }

    fn change_dir(&mut self, dir: Option<Direction>) {
        let dir = match dir {
            Some(dir) => dir,
            None => match self.dir {
                Some(dir) => dir,
                None => return,
            },
        };

        match self.dir {
            Some(Direction::Up) => {
                self.y -= 1;
                if dir != Direction::Down {
                    self.dir = Some(dir);
                }
            }
            Some(Direction::Down) => {
                self.y += 1;
                if dir != Direction::Up {
                    self.dir = Some(dir);
                }
            }
            Some(Direction::Left) => {
                self.x -= 1;
                if dir != Direction::Right {
                    self.dir = Some(dir);
                }
            }
            Some(Direction::Right) => {
                self.x += 1;
                if dir != Direction::Left {
                    self.dir = Some(dir);
                }
            }
            None => self.dir = Some(dir),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FoodCell {
    pub x: u32,
    pub y: u32,
}

impl FoodCell {
    pub fn new(x: u32, y: u32) -> Self {
        Self { x, y }
    }
}

pub struct Game {
    pub snake: Snake,
    pub food: FoodCell,
    pub width: u32,
    pub height: u32,
    pub score: u32,
    pub state: GameState,
}

impl Game {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            snake: Snake {
                body: vec![SnakeCell::new(2, 1), SnakeCell::new(1, 2)],
            },
            food: FoodCell::new(5, 5),
            width,
            height,
            score: 0,
            state: GameState::Playing,
        }
    }

    pub fn step(&mut self, turn: Option<Direction>) {
        let mut cloned = self.snake.body.clone();
        for (i, cell) in self.snake.body.iter_mut().enumerate().rev() {
            cell.change_dir(if i == 0 { turn } else { cloned[i - 1].dir });
        }
        if self.snake.body[0].into() == self.food {
            self.score += 1;
            match self.gen_non_overlapping() {
                Some((x, y)) => {
                    self.food = FoodCell::new(x, y);
                    self.snake.body.push(cloned.pop().unwrap());
                }
                None => {
                    self.state = GameState::Won;
                    return;
                }
            }
        } else if self.snake.body[0].x >= self.width
            || self.snake.body[0].y >= self.height
            || self.snake.body[0].x == 0
            || self.snake.body[0].y == 0
            || self
                .snake
                .body
                .iter()
                .skip(1)
                .any(|c| c == &self.snake.body[0])
        {
            self.state = GameState::Lost;
        }
    }

    fn gen_non_overlapping(&self) -> Option<(u32, u32)> {
        let mut empty = (1..self.width)
            .map(|x| (1..self.height).map(move |y| (x, y)))
            .flatten()
            .collect::<Vec<_>>();
        self.snake.body.iter().for_each(|c| {
            if let Some(i) = empty.iter().position(|&(x, y)| x == c.x && y == c.y) {
                empty.remove(i);
            }
        });
        empty.choose(&mut rand::thread_rng()).cloned()
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GameState {
    Lost,
    Won,
    Playing,
}
