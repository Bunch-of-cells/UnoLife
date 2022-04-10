use std::time::Instant;

use super::{Direction, Game, GameState};
use crate::components::application::{MiniApp, DEFAULT_HEIGHT, DEFAULT_WIDTH};
use crate::components::button::{draw_text, Pos};
use crate::menu::{config::Config, highscores::HighScores, ui::TOP_PAD};
use crate::Event;
use piston_window::*;

const GRID_SIZE: u32 = 40;
const FPS: u64 = 20;

pub struct SnakeApp {
    game: Game,
    dir: Option<Direction>,
    size: f64,
    now: Option<Instant>,
}

impl SnakeApp {
    pub fn new() -> Self {
        SnakeApp {
            game: Game::new(GRID_SIZE, GRID_SIZE),
            dir: None,
            size: (DEFAULT_HEIGHT as f64 - TOP_PAD) / GRID_SIZE as f64,
            now: None,
        }
    }
}

impl MiniApp for SnakeApp {
    fn render(
        &mut self,
        window: &mut PistonWindow,
        event: &Event,
        glyphs: &mut Glyphs,
        _config: &mut Config,
        _highscores: &mut HighScores,
    ) {
        self.dir = if let Some(Button::Keyboard(press)) = event.press_args() {
            match press {
                Key::Up => Some(Direction::Up),
                Key::Down => Some(Direction::Down),
                Key::Left => Some(Direction::Left),
                Key::Right => Some(Direction::Right),
                _ => None,
            }
        } else {
            self.dir
        };

        if self.dir.is_some() && self.now.is_none() {
            self.now = Some(Instant::now());
        }

        window.draw_2d(event, |c, g, device| {
            clear([1.0; 4], g);

            let ctx = c.trans((DEFAULT_WIDTH as f64 - (DEFAULT_HEIGHT as f64 - TOP_PAD)) / 2.0, TOP_PAD);

            match self.game.state {
                GameState::Playing => {
                    if matches!(self.now, Some(now) if now.elapsed().as_millis() >= (1000 / FPS) as u128) {
                        self.game.step(self.dir);
                        self.now = Some(Instant::now());
                    }
                }
                GameState::Lost => draw_text(
                    &c,
                    g,
                    glyphs,
                    [0.0, 0.0, 0.0, 1.0],
                    Pos { x: 10.0, y: 528.0 },
                    "Loser",
                    28,
                ),
                GameState::Won => draw_text(
                    &c,
                    g,
                    glyphs,
                    [0.0, 0.0, 0.0, 1.0],
                    Pos { x: 10.0, y: 528.0 },
                    "Winner",
                    28,
                ),
            }

            for cell in &self.game.snake.body {
                let x = cell.x as f64 * self.size;
                let y = cell.y as f64 * self.size;

                rectangle(
                    [0.0, 0.0, 0.0, 1.0],
                    [x, y, self.size, self.size],
                    ctx.transform,
                    g,
                );
            }

            rectangle(
                [1.0, 0.0, 0.0, 1.0],
                [
                    self.game.food.x as f64 * self.size,
                    self.game.food.y as f64 * self.size,
                    self.size,
                    self.size,
                ],
                ctx.transform,
                g,
            );

            // for (x, y) in
            //     (0..=self.game.width).flat_map(|x| (0..=self.game.height).map(move |y| (x, y)))
            // {
            //     let rect = [
            //             self.size * (x as f64),
            //             self.size * (y as f64),
            //             self.size,
            //             self.size,
            //         ];
            //     if x == 0 || x == self.game.width || y == 0 || y == self.game.height {
            //         Rectangle::new([0.0, 0.0, 0.0, 1.0]).draw(
            //             rect,
            //             &Default::default(),
            //             ctx.transform,
            //             g,
            //         );
            //     }
            // }

            for x in 0..=self.game.width {
                Line::new([0.0, 0.0, 0.0, 1.0], 0.5).draw(
                    [
                        self.size * (x as f64),
                        self.size * (0 as f64),
                        self.size * (x as f64),
                        self.size * (self.game.height as f64),
                    ],
                    &Default::default(),
                    ctx.transform,
                    g,
                );
            }

            for y in 0..=self.game.height {
                Line::new([0.0, 0.0, 0.0, 1.0], 0.5).draw(
                    [
                        self.size * (0 as f64),
                        self.size * (y as f64),
                        self.size * (self.game.width as f64),
                        self.size * (y as f64),
                    ],
                    &Default::default(),
                    ctx.transform,
                    g,
                );
            }

            // Update glyphs before rendering
            glyphs.factory.encoder.flush(device);
        });
    }
}
