pub const HEIGHT: f32 = 600.0;
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
pub const TOP_WALL: f32 = 1.0;
pub const BOTTOM_WALL: f32 = HEIGHT - 10.0;

const BRICK_SIZE: [f32; 2] = [100.0, 30.0];

// const PADDLE_PADDING: f32 = 10.0;
const GAP_BETWEEN_PADDLE_AND_FLOOR: f32 = 200.0;
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

pub struct Paddle(pub Rect);
pub struct Brick(pub Rect);

pub struct Ball {
    velocity: [f32; 2],
    pub rect: Rect,
}

impl Brick {
    fn generate_bricks() -> Vec<Brick> {
        let mut bricks = Vec::new();
        let mut x;
        let mut y = BOTTOM_WALL - (GAP_BETWEEN_PADDLE_AND_BRICKS + GAP_BETWEEN_PADDLE_AND_FLOOR);
        while y > GAP_BETWEEN_BRICKS_AND_CEILING {
            x = GAP_BETWEEN_BRICKS_AND_SIDES;
            while x < (RIGHT_WALL - GAP_BETWEEN_BRICKS_AND_SIDES) {
                bricks.push(Brick(Rect {
                    x,
                    y,
                    h: BRICK_SIZE[1],
                    w: BRICK_SIZE[0],
                }));
                x += BRICK_SIZE[0] + GAP_BETWEEN_BRICKS;
            }
            y -= BRICK_SIZE[1] + GAP_BETWEEN_BRICKS;
        }
        bricks
    }
}

impl Paddle {
    fn new() -> Self {
        Paddle(Rect {
            x: PADDLE_INITIAL_POSITION[0],
            y: PADDLE_INITIAL_POSITION[1],
            h: PADDLE_SIZE[1],
            w: PADDLE_SIZE[0],
        })
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
    pub paddle: Paddle,
    pub ball: Ball,
    pub bricks: Vec<Brick>,
    pub last_update: Instant,
}

impl Game {
    pub fn new() -> Game {
        let mut game = Game {
            paddle: Paddle::new(),
            ball: Ball::new(),
            bricks: Brick::generate_bricks(),
            last_update: Instant::now(),
        };
        game.update(None);
        game
    }

    pub fn update(&mut self, paddle_movement: Option<HorizontalMovement>) {
        let dt = self.last_update.elapsed().as_secs_f32();
        if let Some(movement) = paddle_movement {
            self.paddle.0.x += (PADDLE_SPEED * dt) * movement as i32 as f32;
        }
        self.ball.rect.x += self.ball.velocity[0] * dt;
        self.ball.rect.y += self.ball.velocity[1] * dt;
        if self.ball.rect.x < LEFT_WALL
            || self.ball.rect.x > RIGHT_WALL
            || self.ball.rect.y < TOP_WALL
            || self.ball.rect.y > BOTTOM_WALL
            // Check if ball colided with paddle
            || (self.ball.rect.collided(&self.paddle.0))
        {
            let rnd = Bernoulli::new(0.5).unwrap();
            if rnd.sample(&mut rand::thread_rng()) {
                self.ball.velocity[0] = -self.ball.velocity[0];
            }
            if rnd.sample(&mut rand::thread_rng()) {
                self.ball.velocity[1] = -self.ball.velocity[1];
            }
        }
        if let Some(brick) = self
            .bricks
            .iter()
            .position(|brick| brick.0.collided(&self.ball.rect))
        {
            self.bricks.remove(brick);
        }
        self.last_update = Instant::now();
    }

    pub fn reset(&mut self) {
        self.ball = Ball::new();
        self.paddle = Paddle::new();
        self.bricks = Brick::generate_bricks();
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
