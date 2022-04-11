use rand::prelude::IteratorRandom;

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

impl Direction {
    fn invert(&self) -> Self {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
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
        match (self.dir, dir) {
            (Some(d), Some(dir)) if d != dir.invert() => self.dir = Some(dir),
            (None, _) => self.dir = dir,
            _ => (),
        }

        match self.dir {
            Some(Direction::Up) => self.y -= 1,
            Some(Direction::Down) => self.y += 1,
            Some(Direction::Left) => self.x -= 1,
            Some(Direction::Right) => self.x += 1,
            None => (),
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
        let mut obj = Self {
            snake: Snake {
                body: vec![SnakeCell::new(1, 1)],
            },
            food: FoodCell::new(5, 5),
            width,
            height,
            score: 0,
            state: GameState::Playing,
        };

        match obj.gen_non_overlapping() {
            Some((x, y)) => obj.food = FoodCell::new(x, y),
            None => obj.state = GameState::Won,
        }

        obj
    }

    pub fn step(&mut self, turn: Option<Direction>) {
        if self.state != GameState::Playing {
            return;
        }
        match turn.or(self.snake.body[0].dir) {
            Some(Direction::Up) if self.snake.body[0].y == 1 => {
                return self.state = GameState::Lost
            }
            Some(Direction::Down) if self.snake.body[0].y == self.height => {
                return self.state = GameState::Lost
            }
            Some(Direction::Left) if self.snake.body[0].x == 1 => {
                return self.state = GameState::Lost
            }
            Some(Direction::Right) if self.snake.body[0].x == self.width => {
                return self.state = GameState::Lost
            }
            _ => (),
        }
        let mut cloned = self.snake.body.clone();
        for (i, cell) in self.snake.body.iter_mut().enumerate().rev() {
            cell.change_dir(if i == 0 {
                turn
            } else {
                cloned[i - 1].dir.or(turn)
            });
        }
        if self.snake.body[0].into() == self.food {
            self.score += 1;
            match self.gen_non_overlapping() {
                Some((x, y)) => {
                    self.food = FoodCell::new(x, y);
                    self.snake.body.push(cloned.pop().unwrap());
                }
                None => self.state = GameState::Won,
            }
        } else if self.snake.body[0].x > self.width
            || self.snake.body[0].y > self.height
            || self.snake.body[0].x == 0
            || self.snake.body[0].y == 0
            || self
                .snake
                .body
                .iter()
                .skip(1)
                .any(|c| c.x == self.snake.body[0].x && c.y == self.snake.body[0].y)
        {
            self.state = GameState::Lost;
        }
    }

    fn gen_non_overlapping(&self) -> Option<(u32, u32)> {
        (1..=self.width)
            .flat_map(|x| (1..=self.height).map(move |y| (x, y)))
            .filter(|(x, y)| !self.snake.body.iter().any(|c| c.x == *x && c.y == *y))
            .choose(&mut rand::thread_rng())
    }

    pub fn reset(&mut self) {
        self.snake.body = vec![SnakeCell::new(1, 1)];
        self.score = 0;
        self.food = match self.gen_non_overlapping() {
            Some((x, y)) => FoodCell::new(x, y),
            None => return self.state = GameState::Won,
        };
        self.state = GameState::Playing;
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GameState {
    Lost,
    Won,
    Playing,
}
