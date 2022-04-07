use super::{CharGuess, Game, GuessResult, GuessType};
use crate::components::application::{MiniApp, DEFAULT_HEIGHT, DEFAULT_WIDTH};
use crate::components::button::{draw_text, Pos};
use crate::Event;
use piston_window::*;

pub struct WordleApp {
    state: Game,
    guess: String,
}

impl WordleApp {
    pub fn new() -> Self {
        WordleApp {
            state: Game::new(),
            guess: String::new(),
        }
    }
}

const BOARD_SIZE: f64 = DEFAULT_HEIGHT as f64 - 100.0;
const CENTER_X: f64 = (DEFAULT_WIDTH as f64 - BOARD_SIZE) / 2.0;
const TOP_PAD: f64 = 104.0;
const SQUARE_SIZE: f64 = BOARD_SIZE / 6.5;

// Converts Guess to Color
fn guess_to_clr(guess: CharGuess) -> [f32; 4] {
    match guess.type_ {
        GuessType::Correct => [160.0 / 255.0, 237.0 / 255.0, 138.0 / 255.0, 1.0],
        GuessType::OutOfOrder => [233.0 / 255.0, 138.0 / 255.0, 237.0 / 255.0, 1.0],
        GuessType::Incorrect => [250.0 / 255.0, 246.0 / 255.0, 188.0 / 255.0, 1.0],
    }
}

impl MiniApp for WordleApp {
    fn render(&mut self, window: &mut PistonWindow, event: &Event, glyphs: &mut Glyphs) {
        let mut text = None;
        if let Some(Button::Keyboard(press)) = event.press_args() {
            match press {
                Key::Backspace | Key::Delete => {
                    self.guess.pop();
                }
                Key::Return => {
                    let result = self.state.guess(self.guess.clone());
                    match result {
                        Err(error) => {
                            text = Some(error.to_string());
                        }
                        Ok(res) => {
                            match res {
                                GuessResult::GameOver(word) => {
                                    text = Some(format!(
                                        "Ran outta tries, try next time bud, word was {word}"
                                    ));
                                }
                                GuessResult::Right => {
                                    text = Some("Ya got it champ".to_string());
                                }
                                _ => (),
                            }
                            self.guess.clear();
                        }
                    }
                }
                _ => {
                    if self.guess.len() < 5 {
                        let character: char = unsafe { std::mem::transmute(press) };
                        self.guess.push(character);
                    }
                }
            }
        }

        window.draw_2d(event, |c, g, device| {
            clear([1.0; 4], g);

            if let Some(ref text) = text {
                println!("{text}");
                draw_text(
                    &c,
                    g,
                    glyphs,
                    [0.0, 0.0, 0.0, 1.0],
                    Pos { x: 450.0, y: 528.0 },
                    text,
                    32,
                );
            }

            // Draw the board
            let ctx = c.trans(CENTER_X + 80.0, TOP_PAD);

            for (y, guesses) in self.state.guesses().iter().enumerate() {
                if let Some(guesses) = guesses {
                    for (x, char_guess) in guesses.result().iter().enumerate() {
                        let clr = guess_to_clr(*char_guess);
                        let rect = math::margin_rectangle(
                            [
                                SQUARE_SIZE * (x as f64),
                                SQUARE_SIZE * (y as f64),
                                SQUARE_SIZE,
                                SQUARE_SIZE,
                            ],
                            4.0,
                        );
                        rectangle(clr, rect, ctx.transform, g);
                        draw_text(
                            &c,
                            g,
                            glyphs,
                            [0.0, 0.0, 0.0, 1.0],
                            Pos { x: x as f64, y: y as f64 },
                            &char_guess.char.to_string(),
                            18,
                        );
                    }
                } else {
                    for x in 0..5 {
                        let rect = math::margin_rectangle(
                            [
                                SQUARE_SIZE * (x as f64),
                                SQUARE_SIZE * (y as f64),
                                SQUARE_SIZE,
                                SQUARE_SIZE,
                            ],
                            4.0,
                        );
                        rectangle([150.0 / 255.0, 246.0 / 255.0, 188.0 / 255.0, 1.0], rect, ctx.transform, g);
                    }
                }
            }

            // Update glyphs before rendering
            glyphs.factory.encoder.flush(device);
        });
    }
}
