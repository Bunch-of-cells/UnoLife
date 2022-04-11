// Defines the amount of time that should elapse between each physics step.
const TIME_STEP: f32 = 1.0 / 60.0;

// These constants are defined in `Transform` units.
// Using the default 2D camera they correspond 1:1 with screen pixels.
// The `const_vec3!` macros are needed as functions that operate on floats cannot be constant in Rust.
const PADDLE_SIZE: [f32; 3] = [120.0, 20.0, 0.0];
const GAP_BETWEEN_PADDLE_AND_FLOOR: f32 = 60.0;
const PADDLE_SPEED: f32 = 500.0;
// How close can the paddle get to the wall
const PADDLE_PADDING: f32 = 10.0;

// We set the z-value of the ball to 1 so it renders on top in the case of overlapping sprites.
const BALL_STARTING_POSITION: [f32; 3] = [0.0, -50.0, 1.0];
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

const BRICK_SIZE: [f32; 2] = [100., 30.];
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


struct Paddle {
    x: f32,
    y: f32,
}

struct Ball {
    velocity: (f32, f32),
    x: f32,
    y: f32,
}

struct Brick;

pub struct Game {
    paddle: Paddle,
    ball: Ball,
    bricks: Vec<Brick>,
}


