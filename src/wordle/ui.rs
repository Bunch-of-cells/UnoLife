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
                Key::A => {
                    if self.guess.len() < 5 {
                        self.guess.push('a')
                    }
                }
                Key::B => {
                    if self.guess.len() < 5 {
                        self.guess.push('b')
                    }
                }
                Key::C => {
                    if self.guess.len() < 5 {
                        self.guess.push('c')
                    }
                }
                Key::D => {
                    if self.guess.len() < 5 {
                        self.guess.push('d')
                    }
                }
                Key::E => {
                    if self.guess.len() < 5 {
                        self.guess.push('e')
                    }
                }
                Key::F => {
                    if self.guess.len() < 5 {
                        self.guess.push('f')
                    }
                }
                Key::G => {
                    if self.guess.len() < 5 {
                        self.guess.push('g')
                    }
                }
                Key::H => {
                    if self.guess.len() < 5 {
                        self.guess.push('h')
                    }
                }
                Key::I => {
                    if self.guess.len() < 5 {
                        self.guess.push('i')
                    }
                }
                Key::J => {
                    if self.guess.len() < 5 {
                        self.guess.push('j')
                    }
                }
                Key::K => {
                    if self.guess.len() < 5 {
                        self.guess.push('k')
                    }
                }
                Key::L => {
                    if self.guess.len() < 5 {
                        self.guess.push('l')
                    }
                }
                Key::M => {
                    if self.guess.len() < 5 {
                        self.guess.push('m')
                    }
                }
                Key::N => {
                    if self.guess.len() < 5 {
                        self.guess.push('n')
                    }
                }
                Key::O => {
                    if self.guess.len() < 5 {
                        self.guess.push('o')
                    }
                }
                Key::P => {
                    if self.guess.len() < 5 {
                        self.guess.push('p')
                    }
                }
                Key::Q => {
                    if self.guess.len() < 5 {
                        self.guess.push('q')
                    }
                }
                Key::R => {
                    if self.guess.len() < 5 {
                        self.guess.push('r')
                    }
                }
                Key::S => {
                    if self.guess.len() < 5 {
                        self.guess.push('s')
                    }
                }
                Key::T => {
                    if self.guess.len() < 5 {
                        self.guess.push('t')
                    }
                }
                Key::U => {
                    if self.guess.len() < 5 {
                        self.guess.push('u')
                    }
                }
                Key::V => {
                    if self.guess.len() < 5 {
                        self.guess.push('v')
                    }
                }
                Key::W => {
                    if self.guess.len() < 5 {
                        self.guess.push('w')
                    }
                }
                Key::X => {
                    if self.guess.len() < 5 {
                        self.guess.push('x')
                    }
                }
                Key::Y => {
                    if self.guess.len() < 5 {
                        self.guess.push('y')
                    }
                }
                Key::Z => {
                    if self.guess.len() < 5 {
                        self.guess.push('z')
                    }
                }
                _ => (),
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

            for (y, guesses) in self.state.guesses().iter().flatten().enumerate() {
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
            }

            // Update glyphs before rendering
            glyphs.factory.encoder.flush(device);
        });
    }
}
