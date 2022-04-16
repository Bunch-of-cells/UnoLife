pub const HEIGHT: f32 = 400.0;
pub const WDITH: f32 = 800.0;

const PADDLE_INITIAL_POSITION: [f32; 2] = [
    (WDITH - PADDLE_SIZE[0]) / 2.0,
    HEIGHT - PADDLE_SIZE[1] - GAP_BETWEEN_PADDLE_AND_FLOOR,
];
const PADDLE_SPEED: f32 = 500.0;
const PADDLE_SIZE: [f32; 2] = [100.0, 20.0];

const BALL_STARTING_POSITION: [f32; 2] = [40.0, 40.0];
const INITIAL_BALL_DIRECTION: [f32; 2] = [5.0, -5.0];
const BALL_SIZE: f32 = 50.0;

pub const RIGHT_WALL: f32 = WDITH - 10.0;
pub const LEFT_WALL: f32 = 10.0;
pub const TOP_WALL: f32 = 10.0;
pub const BOTTOM_WALL: f32 = HEIGHT - 10.0;

const BRICK_SIZE: [f32; 2] = [100.0, 30.0];

// const PADDLE_PADDING: f32 = 10.0;
const GAP_BETWEEN_PADDLE_AND_FLOOR: f32 = 50.0;
const GAP_BETWEEN_PADDLE_AND_BRICKS: f32 = 150.0;
const GAP_BETWEEN_BRICKS: f32 = 50.0;
const GAP_BETWEEN_BRICKS_AND_CEILING: f32 = 20.0;
const GAP_BETWEEN_BRICKS_AND_SIDES: f32 = 20.0;

// use std::time::Instant;

use rand::{distributions::Bernoulli, prelude::Distribution};

pub struct Ball {
    velocity: [f32; 2],
    pub rect: Rect,
}

fn generate_bricks() -> Vec<Rect> {
    let mut bricks = Vec::new();
    let mut x;
    let mut y = TOP_WALL + GAP_BETWEEN_BRICKS_AND_CEILING;
    while y < BOTTOM_WALL - (GAP_BETWEEN_PADDLE_AND_BRICKS + GAP_BETWEEN_PADDLE_AND_FLOOR) {
        x = LEFT_WALL + GAP_BETWEEN_BRICKS_AND_SIDES;
        while x < (RIGHT_WALL - GAP_BETWEEN_BRICKS_AND_SIDES) {
            bricks.push(Rect {
                x,
                y,
                h: BRICK_SIZE[1],
                w: BRICK_SIZE[0],
            });
            x += BRICK_SIZE[0] + GAP_BETWEEN_BRICKS;
        }
        y += BRICK_SIZE[1] + GAP_BETWEEN_BRICKS;
    }
    bricks
}

impl Rect {
    fn new_paddle() -> Self {
        Rect {
            x: PADDLE_INITIAL_POSITION[0],
            y: PADDLE_INITIAL_POSITION[1],
            h: PADDLE_SIZE[1],
            w: PADDLE_SIZE[0],
        }
    }
}

impl Ball {
    fn new() -> Self {
        Ball {
            velocity: INITIAL_BALL_DIRECTION,
            rect: Rect {
                x: BALL_STARTING_POSITION[0],
                y: BALL_STARTING_POSITION[1],
                h: BALL_SIZE,
                w: BALL_SIZE,
            },
        }
    }
}

pub struct Game {
    pub paddle: Rect,
    pub ball: Ball,
    pub bricks: Vec<Rect>,
    // pub last_update: Instant,
}

impl Game {
    pub fn new() -> Game {
        let mut game = Game {
            paddle: Rect::new_paddle(),
            ball: Ball::new(),
            bricks: generate_bricks(),
            // last_update: Instant::now(),
        };
        game.update(None);
        game
    }

    pub fn update(&mut self, paddle_movement: Option<HorizontalMovement>) {
        // let dt = self.last_update.elapsed().as_secs_f32();
        let dt = 1.0;
        if let Some(movement) = paddle_movement {
            self.paddle.x += (PADDLE_SPEED * dt) * movement as i32 as f32;
        }
        self.ball.rect.x += self.ball.velocity[0] * dt;
        self.ball.rect.y += self.ball.velocity[1] * dt;
        if self.ball.rect.x < LEFT_WALL || self.ball.rect.x > RIGHT_WALL {
            self.ball.velocity[0] = -self.ball.velocity[0];
        } else if self.ball.rect.y < TOP_WALL || self.ball.rect.y > BOTTOM_WALL {
            self.ball.velocity[1] = -self.ball.velocity[1];
        } else if self.ball.rect.collided(&self.paddle) {
            let rnd = Bernoulli::new(0.5).unwrap();
            match [0; 2].map(|_| rnd.sample(&mut rand::thread_rng())) {
                [a, b] if a == b => {
                    self.ball.velocity[0] = -self.ball.velocity[0];
                    self.ball.velocity[1] = -self.ball.velocity[1];
                }
                [true, _] => self.ball.velocity[0] = -self.ball.velocity[0],
                [false, _] => self.ball.velocity[1] = -self.ball.velocity[1],
            }
        }
        if let Some(brick) = self
            .bricks
            .iter()
            .position(|brick| brick.collided(&self.ball.rect))
        {
            self.bricks.remove(brick);
        }
        // self.last_update = Instant::now();
    }

    pub fn reset(&mut self) {
        self.ball = Ball::new();
        self.paddle = Rect::new_paddle();
        self.bricks = generate_bricks();
    }
}

pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

impl Rect {
    /// Returns true if self collided with other from any direction
    fn collided(&self, other: &Rect) -> bool {
        self.x < other.x + other.w
            && self.x + self.w > other.x
            && self.y < other.y + other.h
            && self.y + self.h > other.y
    }
}

pub enum HorizontalMovement {
    Left = -1,
    Right = 1,
}
