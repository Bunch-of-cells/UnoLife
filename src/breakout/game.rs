const TIME_STEP: u128 = 0;
pub const PADDLE_SIZE: [f32; 2] = [50.0, 20.0];
const PADDLE_INITIAL_POSITION: [f32; 2] = [
    (DEFAULT_WIDTH as f32 - PADDLE_SIZE[0]) / 2.0,
    DEFAULT_HEIGHT as f32 - PADDLE_SIZE[1] - 200.0,
];
// const GAP_BETWEEN_PADDLE_AND_FLOOR: f32 = 60.0;
const PADDLE_SPEED: f32 = 500.0;
pub const BALL_SIZE: f32 = 50.0;
// const PADDLE_PADDING: f32 = 10.0;

const BALL_STARTING_POSITION: [f32; 2] = [40.0, 40.0];
const INITIAL_BALL_DIRECTION: [f32; 2] = [5.0, -5.0];

pub const RIGHT_WALL: f32 = DEFAULT_WIDTH as f32 - 10.0;
pub const LEFT_WALL: f32 = 10.0;
pub const TOP_WALL: f32 = 1.0;
pub const BOTTOM_WALL: f32 = DEFAULT_HEIGHT as f32 - 10.0 - TOP_PAD as f32;

pub const BRICK_SIZE: [f32; 2] = [100., 30.];

const GAP_BETWEEN_PADDLE_AND_BRICKS: f32 = 300.0;
const GAP_BETWEEN_BRICKS: f32 = 50.0;

const GAP_BETWEEN_BRICKS_AND_CEILING: f32 = 20.0;
const GAP_BETWEEN_BRICKS_AND_SIDES: f32 = 20.0;

// const BACKGROUND_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);
// const PADDLE_COLOR: Color = Color::rgb(0.3, 0.3, 0.7);
// const BALL_COLOR: Color = Color::rgb(1.0, 0.5, 0.5);
// const BRICK_COLOR: Color = Color::rgb(0.5, 0.5, 1.0);
// const WALL_COLOR: Color = Color::rgb(0.8, 0.8, 0.8);

use std::time::Instant;

use rand::{distributions::Bernoulli, prelude::Distribution};

use crate::{
    components::application::{DEFAULT_HEIGHT, DEFAULT_WIDTH},
    menu::ui::TOP_PAD,
};

pub struct Paddle {
    pub x: f32,
    pub y: f32,
}

pub struct Ball {
    velocity: [f32; 2],
    pub x: f32,
    pub y: f32,
}

pub struct Brick {
    pub x: f32,
    pub y: f32,
}

impl Brick {
    fn generate_bricks() -> Vec<Brick> {
        let mut bricks = Vec::new();
        let mut x;
        let mut y = GAP_BETWEEN_PADDLE_AND_BRICKS;
        while y > GAP_BETWEEN_BRICKS_AND_CEILING {
            x = GAP_BETWEEN_BRICKS_AND_SIDES;
            while x < RIGHT_WALL {
                bricks.push(Brick { x, y });
                x += BRICK_SIZE[0] + GAP_BETWEEN_BRICKS;
            }
            y -= BRICK_SIZE[1] + GAP_BETWEEN_BRICKS;
        }
        bricks
    }
}

pub struct Game {
    pub paddle: Paddle,
    pub ball: Ball,
    pub bricks: Vec<Brick>,
    pub last_update: Instant,
}

impl Game {
    pub fn new() -> Game {
        Game {
            paddle: Paddle {
                x: PADDLE_INITIAL_POSITION[0],
                y: PADDLE_INITIAL_POSITION[1],
            },
            ball: Ball {
                velocity: INITIAL_BALL_DIRECTION,
                x: BALL_STARTING_POSITION[0],
                y: BALL_STARTING_POSITION[1],
            },
            bricks: Brick::generate_bricks(),
            last_update: Instant::now(),
        }
    }

    pub fn update(&mut self, paddle_movement: Option<HorizontalMovement>) {
        let elapsed = self.last_update.elapsed();
        if elapsed.as_millis() < TIME_STEP {
            return;
        }
        let dt = elapsed.as_secs_f32();
        if let Some(movement) = paddle_movement {
            self.paddle.x += (PADDLE_SPEED * dt) * movement as i32 as f32;
        }
        self.ball.x += self.ball.velocity[0] * dt;
        self.ball.y += self.ball.velocity[1] * dt;
        if self.ball.x < LEFT_WALL
            || self.ball.x > RIGHT_WALL
            || self.ball.y < TOP_WALL
            || self.ball.y > BOTTOM_WALL
            // Check if ball colided with paddle
            || (is_in_touch(self.ball.x, self.ball.y, BALL_SIZE, BALL_SIZE, self.paddle.x, self.paddle.y, PADDLE_SIZE[0], PADDLE_SIZE[1]))
        {
            let rnd = Bernoulli::new(0.5).unwrap();
            if rnd.sample(&mut rand::thread_rng()) {
                self.ball.velocity[0] = -self.ball.velocity[0];
            }
            if rnd.sample(&mut rand::thread_rng()) {
                self.ball.velocity[1] = -self.ball.velocity[1];
            }
        }
        if let Some(brick) = self.bricks.iter().position(|brick| {
            is_in_touch(
                self.ball.x,
                self.ball.y,
                BALL_SIZE,
                BALL_SIZE,
                brick.x,
                brick.y,
                BRICK_SIZE[0],
                BRICK_SIZE[1],
            )
        }) {
            self.bricks.remove(brick);
        }
        self.last_update = Instant::now();
    }

    pub fn reset(&mut self) {
        self.paddle.x = PADDLE_INITIAL_POSITION[0];
        self.paddle.y = PADDLE_INITIAL_POSITION[1];
        self.ball.x = BALL_STARTING_POSITION[0];
        self.ball.y = BALL_STARTING_POSITION[1];
        self.ball.velocity = INITIAL_BALL_DIRECTION;
        self.bricks = Brick::generate_bricks();
    }
}

fn is_in_touch(x1: f32, y1: f32, w1: f32, h1: f32, x2: f32, y2: f32, w2: f32, h2: f32) -> bool {
    x1 > x2 && x1 < x2 + w2 && y1 > y2 && y1 < y2 + h2
        || x1 + w1 > x2 && x1 < x2 + w2 && y1 + h1 > y2 && y1 < y2 + h2
}

pub enum HorizontalMovement {
    Left = -1,
    Right = 1,
}
