use super::{Game, Direction, GameState};
use crate::components::application::{MiniApp, DEFAULT_HEIGHT, DEFAULT_WIDTH};
use crate::components::button::{draw_text, Pos, UIButton};
use crate::menu::{config::Config, highscores::HighScores, ui::TOP_PAD};
use crate::Event;
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

const BOARD_SIZE: f64 = DEFAULT_HEIGHT as f64 - 100.0;
const CENTER_X: f64 = (DEFAULT_WIDTH as f64 - BOARD_SIZE) / 2.0;
const SQUARE_SIZE: f64 = BOARD_SIZE / 6.5;

// Converts Guess to Color
// fn guess_to_clr(guess: CharGuess) -> [f32; 4] {
//     match guess.type_ {
//         GuessType::Correct => [77.0 / 255.0, 143.0 / 255.0, 69.0 / 255.0, 1.0],
//         GuessType::OutOfOrder => [212.0 / 255.0, 189.0 / 255.0, 59.0 / 255.0, 1.0],
//         GuessType::Incorrect => [0.5, 0.5, 0.5, 1.0],
//     }
// }

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
            [242.0 / 255.0, 87.0 / 255.0, 87.0 / 255.0, 0.9],
            [1.0, 1.0, 1.0, 1.0],
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
                _ => ()
            }
        }

        window.draw_2d(event, |c, g, device| {
            clear(
                if config.options.white_theme {
                    [1.0; 4]
                } else {
                    [100. / 255., 100. / 255., 100. / 255., 1.0]
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
                    [0.0, 0.0, 0.0, 1.0]
                } else {
                    [1.0, 1.0, 1.0, 1.0]
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
                    [0.0, 0.0, 0.0, 1.0]
                } else {
                    [1.0, 1.0, 1.0, 1.0]
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
                        highscores.scores.snake = std::cmp::max(
                            highscores.scores.snake,
                            self.game.score,
                        );
                        highscores.save_scores(highscores.location.clone());
                        self.first_result = false;
                    }

                    draw_text(
                        &c,
                        g,
                        glyphs,
                        [242.0 / 255.0, 87.0 / 255.0, 87.0 / 255.0, 1.0],
                        Pos { x: 10.0, y: 200.0 },
                        "Game Over",
                        24,
                    );
                },
                GameState::Won => {
                    // update highscore
                    if self.first_result {
                        highscores.scores.snake = std::cmp::max(
                            highscores.scores.snake,
                            self.game.score,
                        );
                        highscores.save_scores(highscores.location.clone());
                        self.first_result = false;
                    }

                    draw_text(
                        &c,
                        g,
                        glyphs,
                        [43.0 / 255.0, 1.0, 0.0, 1.0],
                        Pos { x: 10.0, y: 200.0 },
                        "You win!",
                        20,
                    );
                }
                GameState::Playing => ()
            }

            // Draw the board
            let ctx = c.trans(CENTER_X + 80.0, TOP_PAD);

            // draw buttons
            reset_button.draw(&c, g, glyphs);

            // Update glyphs before rendering
            glyphs.factory.encoder.flush(device);
        });
    }
}
