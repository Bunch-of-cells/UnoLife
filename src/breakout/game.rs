// Defines the amount of time that should elapse between each physics step.
pub const TIME_STEP: f32 = 1.0 / 60.0;

// These constants are defined in `Transform` units.
// Using the default 2D camera they correspond 1:1 with screen pixels.
// The `const_vec3!` macros are needed as functions that operate on floats cannot be constant in Rust.
pub const PADDLE_SIZE: [f32; 3] = [120.0, 20.0, 0.0];
const GAP_BETWEEN_PADDLE_AND_FLOOR: f32 = 60.0;
const PADDLE_SPEED: f32 = 500.0;
// How close can the paddle get to the wall
const PADDLE_PADDING: f32 = 10.0;

// We set the z-value of the ball to 1 so it renders on top in the case of overlapping sprites.
const BALL_STARTING_POSITION: [f32; 2] = [0.0, -50.0];
const BALL_SIZE: [f32; 3] = [30.0, 30.0, 0.0];
const BALL_SPEED: f32 = 400.0;
const INITIAL_BALL_DIRECTION: [f32; 2] = [0.5, -0.5];

const WALL_THICKNESS: f32 = 10.0;
// x coordinates
const LEFT_WALL: f32 = -450.;
const RIGHT_WALL: f32 = 450.;
// y coordinates
const BOTTOM_WALL: f32 = -300.;
const TOP_WALL: f32 = 300.;

pub const BRICK_SIZE: [f32; 2] = [100., 30.];
// These values are exact
const GAP_BETWEEN_PADDLE_AND_BRICKS: f32 = 270.0;
const GAP_BETWEEN_BRICKS: f32 = 5.0;
// These values are lower bounds, as the number of bricks is computed
const GAP_BETWEEN_BRICKS_AND_CEILING: f32 = 20.0;
const GAP_BETWEEN_BRICKS_AND_SIDES: f32 = 20.0;

// const BACKGROUND_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);
// const PADDLE_COLOR: Color = Color::rgb(0.3, 0.3, 0.7);
// const BALL_COLOR: Color = Color::rgb(1.0, 0.5, 0.5);
// const BRICK_COLOR: Color = Color::rgb(0.5, 0.5, 1.0);
// const WALL_COLOR: Color = Color::rgb(0.8, 0.8, 0.8);

use rand::{distributions::Bernoulli, prelude::Distribution};

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
        let mut x = GAP_BETWEEN_BRICKS_AND_SIDES;
        let mut y = GAP_BETWEEN_BRICKS_AND_CEILING;
        let mut row = 0;
        while y < TOP_WALL {
            x = GAP_BETWEEN_BRICKS_AND_SIDES;
            while x < RIGHT_WALL {
                bricks.push(Brick { x, y });
                x += BRICK_SIZE[0] + GAP_BETWEEN_BRICKS;
            }
            y += BRICK_SIZE[1] + GAP_BETWEEN_BRICKS;
            row += 1;
        }
        bricks
    }
}

pub struct Game {
    pub paddle: Paddle,
    pub ball: Ball,
    pub bricks: Vec<Brick>,
}

impl Game {
    pub fn new() -> Game {
        Game {
            paddle: Paddle { x: 0.0, y: 0.0 },
            ball: Ball {
                velocity: INITIAL_BALL_DIRECTION,
                x: BALL_STARTING_POSITION[0],
                y: BALL_STARTING_POSITION[1],
            },
            bricks: Brick::generate_bricks(),
        }
    }

    pub fn update(&mut self, dt: f32, paddle_movement: Option<HorizontalMovement>) {
        if let Some(movement) = paddle_movement {
            self.paddle.x += (PADDLE_SPEED * dt) * movement as i32 as f32;
        }
        self.ball.x += self.ball.velocity[0] * dt;
        self.ball.y += self.ball.velocity[1] * dt;
        if self.ball.x < 0.0
            || self.ball.x > RIGHT_WALL
            || self.ball.y > 0.0
            || self.ball.y < TOP_WALL
            // Check if ball colided with paddle
            || (self.ball.x > self.paddle.x
                && self.ball.x < self.paddle.x + PADDLE_SIZE[0]
                && self.ball.y > self.paddle.y
                && self.ball.y < self.paddle.y + PADDLE_SIZE[1])
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
            self.ball.x > brick.x
                && self.ball.x < brick.x + BRICK_SIZE[0]
                && self.ball.y > brick.y
                && self.ball.y < brick.y + BRICK_SIZE[1]
        }) {
            self.bricks.remove(brick);
        }
    }

    pub fn reset(&mut self) {
        self.paddle.x = 0.0;
        self.paddle.y = 0.0;
        self.ball.x = BALL_STARTING_POSITION[0];
        self.ball.y = BALL_STARTING_POSITION[1];
        self.ball.velocity = INITIAL_BALL_DIRECTION;
        self.bricks = Brick::generate_bricks();
    }
}

pub enum HorizontalMovement {
    Left = -1,
    Right = 1,
}
