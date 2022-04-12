use super::{Direction, Game};
use crate::components::application::{MiniApp, DEFAULT_HEIGHT, DEFAULT_WIDTH};
use crate::components::{
    button::{draw_text, Pos, UIButton},
    color::Color,
};
use crate::menu::{config::Config, highscores::HighScores, ui::TOP_PAD};
use crate::{rgb, Event};
use piston_window::*;

pub struct Puzzle15App {
    game: Game,
    hover_pos: [f64; 2],
    first_result: bool,
}

impl Puzzle15App {
    pub fn new() -> Self {
        Puzzle15App {
            game: Game::new(),
            hover_pos: [0.0, 0.0],
            first_result: true,
        }
    }
}

const BOARD_SIZE: f64 = DEFAULT_HEIGHT as f64 - TOP_PAD;
const CENTER_X: f64 = (DEFAULT_WIDTH as f64 - BOARD_SIZE) / 2.0;
const SQUARE_SIZE: f64 = BOARD_SIZE / 4.5;

impl MiniApp for Puzzle15App {
    fn render(
        &mut self,
        windows: &mut Vec<PistonWindow>,
        event: &Event,
        glyphs: &mut Glyphs,
        config: &mut Config,
        highscores: &mut HighScores,
    ) {
        let window = &mut windows[0];
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
                    Color::DARK_THEME_BG
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
                &format!("Moves: {}", self.game.moves),
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
                &format!("Highscore: {}", highscores.scores.puzzle15),
                28,
            );

            if self.game.is_over() {
                if self.first_result {
                    highscores.scores.puzzle15 =
                        std::cmp::min(highscores.scores.puzzle15, self.game.moves);
                    highscores.save_scores();
                    self.first_result = false;
                }

                draw_text(
                    &c,
                    g,
                    glyphs,
                    Color::WIN_TEXT,
                    Pos { x: 10.0, y: 200.0 },
                    "You win!",
                    20,
                );
            }

            // Draw the board
            let ctx = c.trans(CENTER_X + 40.0, TOP_PAD);

            for (i, &val) in self.game.board.iter().enumerate() {
                let x = i % 4;
                let y = i / 4;
                let rect = math::margin_rectangle(
                    [
                        SQUARE_SIZE * (x as f64),
                        SQUARE_SIZE * (y as f64),
                        SQUARE_SIZE,
                        SQUARE_SIZE,
                    ],
                    4.0,
                );
                Rectangle::new(if val == 0 {
                    rgb!(204, 192, 179)
                } else {
                    rgb!(238, 228, 218)
                })
                .draw(rect, &Default::default(), ctx.transform, g);

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
