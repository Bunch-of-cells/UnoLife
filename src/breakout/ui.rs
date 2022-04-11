use super::{Game, HorizontalMovement, BRICK_SIZE};
use crate::components::application::{MiniApp, DEFAULT_HEIGHT, DEFAULT_WIDTH};
use crate::components::{
    button::{draw_text, Pos, UIButton},
    color::Color,
};
use crate::menu::{config::Config, highscores::HighScores, ui::TOP_PAD};
use crate::{rgb, Event};
use piston_window::*;

pub struct WordleApp {
    state: Game,
    hover_pos: [f64; 2],
}

impl WordleApp {
    pub fn new() -> Self {
        WordleApp {
            state: Game::new(),
            hover_pos: [0.0, 0.0],
        }
    }
}

const BOARD_SIZE: f64 = DEFAULT_HEIGHT as f64 - 100.0;
const CENTER_X: f64 = (DEFAULT_WIDTH as f64 - BOARD_SIZE) / 2.0;
const SQUARE_SIZE: f64 = BOARD_SIZE / 6.5;

// Converts Guess to Color

impl MiniApp for WordleApp {
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
                self.state.reset();
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
                    // black
                    Color::WHITE
                },
                Pos { x: 10.0, y: 400.0 },
                &format!("Win streak: {}", highscores.scores.wordle),
                28,
            );

            // Draw the board
            let ctx = c.trans(CENTER_X + 80.0, TOP_PAD);

            for brick in &self.state.bricks {
                let rect = [
                    (brick.x + BRICK_SIZE[0]) as f64,
                    (brick.y + BRICK_SIZE[1]) as f64,
                    brick.x as f64,
                    brick.y as f64,
                ];
                rectangle(rgb!(200, 200, 100), rect, ctx.transform, g);
            }

            // Update glyphs before rendering
            glyphs.factory.encoder.flush(device);
        });
    }
}
