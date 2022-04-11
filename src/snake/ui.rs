use std::time::Instant;

use super::{Direction, Game, GameState};
use crate::components::application::{MiniApp, DEFAULT_HEIGHT, DEFAULT_WIDTH};
use crate::components::{button::{draw_text, Pos, UIButton}, color::Color};
use crate::menu::{config::Config, highscores::HighScores, ui::TOP_PAD};
use crate::{rgb, Event};
use piston_window::*;

const GRID_SIZE: u32 = 30;
const FPS: u64 = 15;

pub struct SnakeApp {
    game: Game,
    dir: Option<Direction>,
    size: f64,
    now: Option<Instant>,
    first_result: bool,
    hover_pos: [f64; 2],
}

impl SnakeApp {
    pub fn new() -> Self {
        SnakeApp {
            game: Game::new(GRID_SIZE, GRID_SIZE),
            dir: None,
            size: (DEFAULT_HEIGHT as f64 - TOP_PAD - GRID_SIZE as f64 / 2.0) / GRID_SIZE as f64,
            now: None,
            first_result: true,
            hover_pos: [0.0; 2],
        }
    }
}

impl MiniApp for SnakeApp {
    fn render(
        &mut self,
        window: &mut PistonWindow,
        event: &Event,
        glyphs: &mut Glyphs,
        config: &mut Config,
        highscores: &mut HighScores,
    ) {
        if let Some([cx, cy]) = event.mouse_cursor_args() {
            self.hover_pos = [cx, cy];
        }

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

        let left_click = event.press_args() == Some(Button::Mouse(MouseButton::Left));

        // init buttons
        let mut reset_button = UIButton::new(
            "     Reset",
            Color::RESET,
            Color::WHITE,
            24,
            Pos { x: 808.0, y: 145.0 },
            160.0,
            48.0,
        );

        // handle button events
        if reset_button.is_over(self.hover_pos[0], self.hover_pos[1]) {
            if left_click {
                // update highscore
                highscores.scores.snake = std::cmp::max(highscores.scores.snake, self.game.score);
                highscores.save_scores();
                self.first_result = true;

                self.game.reset();
                self.dir = None;
                self.now = None;
            } else {
                reset_button.width += 6.0;
                reset_button.pos.x -= 3.0;
                reset_button.height += 6.0;
                reset_button.pos.y -= 3.0;
                reset_button.size += 1;
            }
        }

        window.draw_2d(event, |c, g, device| {
            clear(
                if config.options.white_theme {
                    Color::WHITE
                } else {
                    rgb!(100, 100, 100)
                },
                g,
            );

            let ctx = c.trans((DEFAULT_WIDTH as f64 - (DEFAULT_HEIGHT as f64 - TOP_PAD)) / 2.0, TOP_PAD - self.size / 2.0);

            draw_text(
                &c,
                g,
                glyphs,
                if config.options.white_theme {
                    Color::BLACK
                } else {
                    Color::WHITE
                },
                Pos { x: 10.0, y: 400.0 },
                &format!("Score: {}", self.game.score),
                28,
            );
            draw_text(
                &c,
                g,
                glyphs,
                if config.options.white_theme {
                    Color::BLACK
                } else {
                    Color::WHITE
                },
                Pos { x: 10.0, y: 440.0 },
                &format!("Highscore: {}", highscores.scores.snake),
                28,
            );

            match self.game.state {
                GameState::Playing => {
                    if matches!(self.now, Some(now) if now.elapsed().as_millis() >= (1000 / FPS) as u128) {
                        self.game.step(self.dir);
                        self.now = Some(Instant::now());
                    }
                }
                GameState::Lost => {
                    // update highscore
                    if self.first_result {
                        highscores.scores.snake = std::cmp::max(
                            highscores.scores.snake,
                            self.game.score,
                        );
                        highscores.save_scores();
                        self.first_result = false;
                    }

                    draw_text(
                        &c,
                        g,
                        glyphs,
                        rgb!(242, 87, 87),
                        Pos { x: 10.0, y: 528.0 },
                        "You lost!",
                        28,
                    )
                },
                GameState::Won => {
                    // update highscore
                    if self.first_result {
                        highscores.scores.snake = std::cmp::max(
                            highscores.scores.snake,
                            self.game.score,
                        );
                        highscores.save_scores();
                        self.first_result = false;
                    }

                    draw_text(
                        &c,
                        g,
                        glyphs,
                        rgb!(43, 255, 0),
                        Pos { x: 10.0, y: 528.0 },
                        "You win!",
                        28,
                    )
                }
            }

            // draw snake
            for cell in &self.game.snake.body {
                let x = cell.x as f64 * self.size;
                let y = cell.y as f64 * self.size;

                rectangle(
                    Color::BLACK,
                    [x, y, self.size, self.size],
                    ctx.transform,
                    g,
                );
            }

            // draw food
            rectangle(
                rgb!(255, 0, 0),
                [
                    self.game.food.x as f64 * self.size,
                    self.game.food.y as f64 * self.size,
                    self.size,
                    self.size,
                ],
                ctx.transform,
                g,
            );

            // draw boundaries
            for (x, y) in (0..=self.game.width + 1).zip(0..=self.game.height + 1) {
                Line::new(Color::BLACK, 0.5).draw(
                    [
                        self.size * (x as f64),
                        0.0,
                        self.size * (x as f64),
                        self.size * (self.game.height + 1) as f64,
                    ],
                    &Default::default(),
                    ctx.transform,
                    g,
                );
                Line::new(Color::BLACK, 0.5).draw(
                    [
                        0.0,
                        self.size * (y as f64),
                        self.size * (self.game.width + 1) as f64,
                        self.size * (y as f64),
                    ],
                    &Default::default(),
                    ctx.transform,
                    g,
                );
            }

            // draw buttons
            reset_button.draw(&c, g, glyphs);

            // Update glyphs before rendering
            glyphs.factory.encoder.flush(device);
        });
    }
}
