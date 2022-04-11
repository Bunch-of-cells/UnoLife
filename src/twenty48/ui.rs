use super::{Direction, Game, GameState};
use crate::components::application::{MiniApp, DEFAULT_HEIGHT, DEFAULT_WIDTH};
use crate::components::{
    button::{draw_text, Pos, UIButton},
    color::Color,
};
use crate::menu::{config::Config, highscores::HighScores, ui::TOP_PAD};
use crate::{rgb, Event};
use piston_window::*;

pub struct Twenty48App {
    game: Game,
    hover_pos: [f64; 2],
    first_result: bool,
}

impl Twenty48App {
    pub fn new() -> Self {
        Twenty48App {
            game: Game::new(),
            hover_pos: [0.0, 0.0],
            first_result: true,
        }
    }
}

const BOARD_SIZE: f64 = DEFAULT_HEIGHT as f64 - TOP_PAD;
const CENTER_X: f64 = (DEFAULT_WIDTH as f64 - BOARD_SIZE) / 2.0;
const SQUARE_SIZE: f64 = BOARD_SIZE / 4.5;

// Converts Guess to Color
fn val_to_clr(val: u32) -> [f32; 4] {
    match val {
        0 => rgb!(204, 192, 179),
        2 => rgb!(238, 228, 218),
        4 => rgb!(237, 224, 200),
        8 => rgb!(242, 177, 121),
        16 => rgb!(245, 149, 99),
        32 => rgb!(246, 124, 95),
        64 => rgb!(246, 94, 59),
        128 => rgb!(237, 207, 114),
        256 => rgb!(237, 204, 97),
        512 => rgb!(237, 200, 80),
        1024 => rgb!(237, 197, 63),
        2048 => rgb!(237, 194, 46),
        _ => Color::WHITE,
    }
}

impl MiniApp for Twenty48App {
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

        // init buttons
        let mut reset_button = UIButton::new(
            "     Reset",
            Color::RESET,
            Color::WHITE,
            24,
            Pos { x: 791.2, y: 135.2 },
            160.0,
            48.0,
        );

        let left_click = event.press_args() == Some(Button::Mouse(MouseButton::Left));

        // handle button events
        if reset_button.is_over(self.hover_pos[0], self.hover_pos[1]) {
            if left_click {
                highscores.scores.twenty48 =
                    std::cmp::max(highscores.scores.twenty48, self.game.score);
                highscores.save_scores();
                self.game.reset();
            } else {
                reset_button.width += 6.0;
                reset_button.pos.x -= 3.0;
                reset_button.height += 6.0;
                reset_button.pos.y -= 3.0;
                reset_button.size += 1;
            }
        }

        if let Some(Button::Keyboard(press)) = event.press_args() {
            match press {
                Key::Up => self.game.step(Direction::Up),
                Key::Down => self.game.step(Direction::Down),
                Key::Left => self.game.step(Direction::Left),
                Key::Right => self.game.step(Direction::Right),
                _ => (),
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

            // draw buttons
            reset_button.draw(&c, g, glyphs);

            // draw highscores
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
                &format!("Highscore: {}", highscores.scores.twenty48),
                28,
            );

            // draw text
            match self.game.state {
                GameState::Lost => {
                    // update highscore
                    if self.first_result {
                        highscores.scores.twenty48 =
                            std::cmp::max(highscores.scores.twenty48, self.game.score);
                        highscores.save_scores();
                        self.first_result = false;
                    }

                    draw_text(
                        &c,
                        g,
                        glyphs,
                        rgb!(242, 87, 87, 0.9),
                        Pos { x: 10.0, y: 200.0 },
                        "Game Over",
                        24,
                    );
                }
                GameState::Won => {
                    // update highscore
                    if self.first_result {
                        highscores.scores.twenty48 =
                            std::cmp::max(highscores.scores.twenty48, self.game.score);
                        highscores.save_scores();
                        self.first_result = false;
                    }

                    draw_text(
                        &c,
                        g,
                        glyphs,
                        rgb!(43, 255, 0),
                        Pos { x: 10.0, y: 200.0 },
                        "You win!",
                        20,
                    );
                }
                GameState::Playing => (),
            }

            // Draw the board
            let ctx = c.trans(CENTER_X + 40.0, TOP_PAD);

            for (i, &val) in self.game.board.iter().enumerate() {
                let x = i % Game::WIDTH;
                let y = i / Game::LENGTH;
                let rect = math::margin_rectangle(
                    [
                        SQUARE_SIZE * (x as f64),
                        SQUARE_SIZE * (y as f64),
                        SQUARE_SIZE,
                        SQUARE_SIZE,
                    ],
                    4.0,
                );
                Rectangle::new(val_to_clr(val)).draw(rect, &Default::default(), ctx.transform, g);
                if val != 0 {
                    draw_text(
                        &ctx,
                        g,
                        glyphs,
                        Color::BLACK,
                        Pos {
                            x: rect[0] + SQUARE_SIZE / 4.0 + 2.0,
                            y: rect[1] + SQUARE_SIZE / 2.0 + 5.0,
                        },
                        &val.to_string(),
                        30,
                    );
                }
            }

            // draw buttons
            reset_button.draw(&c, g, glyphs);

            // Update glyphs before rendering
            glyphs.factory.encoder.flush(device);
        });
    }
}
