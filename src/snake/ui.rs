use super::Game;
use crate::components::application::{MiniApp, DEFAULT_HEIGHT, DEFAULT_WIDTH};
use crate::components::button::{draw_text, Pos, UIButton};
use crate::Event;
use piston_window::*;

pub struct WordleApp {
    state: Game,
    guess: String,
    hover_pos: [f64; 2],
    bg: [f32; 4],
    prev_text: Option<String>,
}

impl WordleApp {
    pub fn new() -> Self {
        WordleApp {
            state: Game::new(),
            guess: String::new(),
            hover_pos: [0.0, 0.0],
            bg: [100. / 255., 100. / 255., 100. / 255., 1.0],
            prev_text: None,
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
        GuessType::Correct => [77.0 / 255.0, 143.0 / 255.0, 69.0 / 255.0, 1.0],
        GuessType::OutOfOrder => [212.0 / 255.0, 189.0 / 255.0, 59.0 / 255.0, 1.0],
        GuessType::Incorrect => [0.5, 0.5, 0.5, 1.0],
    }
}

impl MiniApp for WordleApp {
    fn render(&mut self, window: &mut PistonWindow, event: &Event, glyphs: &mut Glyphs) {
        if let Some([cx, cy]) = event.mouse_cursor_args() {
            self.hover_pos = [cx, cy];
        }

        let mut dark_mode_button = UIButton::new(
            "Switch Mode",
            [0.1, 0.1, 0.1, 1.0],
            [1.0; 4],
            22,
            Pos { x: 791.2, y: 135.2 },
            160.0,
            48.0,
        );

        let left_click = event.press_args() == Some(Button::Mouse(MouseButton::Left));

        if dark_mode_button.is_over(self.hover_pos[0], self.hover_pos[1]) {
            if left_click {
                if self.bg == [1.0; 4] {
                    self.bg = [100. / 255., 100. / 255., 100. / 255., 1.0];
                } else {
                    self.bg = [1.0; 4];
                }
            } else {
                dark_mode_button.width += 6.0;
                dark_mode_button.pos.x -= 3.0;
                dark_mode_button.height += 6.0;
                dark_mode_button.pos.y -= 3.0;
                dark_mode_button.size += 1;
            }
        }

        if let Some(Button::Keyboard(press)) = event.press_args() {
            match press {
                Key::Backspace | Key::Delete => {
                    self.guess.pop();
                }
                Key::Return if self.guess.len() == 5 => {
                    let result = self.state.guess(self.guess.clone());
                    match result {
                        Err(GuessError::GameOver(word)) => {
                            self.prev_text = Some(format!(
                                "Ran outta tries, try next time bud, word was {word}"
                            ));
                        }
                        Err(error) => {
                            self.prev_text = Some(error.to_string());
                        }
                        Ok(res) => {
                            self.prev_text = match res {
                                GuessResult::Right => Some("Ya got it champ".to_string()),
                                GuessResult::Wrong => None,
                            };
                            self.guess.clear();
                        }
                    }
                }
                _ => {
                    if self.guess.len() < 5 {
                        let character: char = unsafe { std::mem::transmute(press) };
                        if character.is_ascii_alphabetic() {
                            self.guess.push(character.to_ascii_uppercase());
                        }
                        self.prev_text = None;
                    }
                }
            }
            // println!("Pressed");
        }
        // let b = now.elapsed().as_millis();

        window.draw_2d(event, |c, g, device| {
            clear(self.bg, g);
            dark_mode_button.draw(&c, g, glyphs);

            if let Some(ref text) = self.prev_text {
                draw_text(
                    &c,
                    g,
                    glyphs,
                    [0.0, 0.0, 0.0, 1.0],
                    Pos { x: 10.0, y: 528.0 },
                    text,
                    28,
                );
            }

            // Draw the board
            let ctx = c.trans(CENTER_X + 80.0, TOP_PAD);

            let mut first = true;
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
                        Rectangle::new_round_border(
                            [211.0 / 255.0, 211.0 / 255.0, 211.0 / 255.0, 1.0],
                            2.0,
                            2.0,
                        )
                        .draw(rect, &Default::default(), ctx.transform, g);
                        Rectangle::new_round(clr, 2.0).draw(
                            rect,
                            &Default::default(),
                            ctx.transform,
                            g,
                        );
                        draw_text(
                            &ctx,
                            g,
                            glyphs,
                            [1.0; 4],
                            Pos {
                                x: rect[0] + SQUARE_SIZE / 4.0 + 2.0,
                                y: rect[1] + SQUARE_SIZE / 2.0 + 5.0,
                            },
                            &char_guess.char.to_string(),
                            30,
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
                        Rectangle::new_round_border(
                            [211.0 / 255.0, 211.0 / 255.0, 211.0 / 255.0, 1.0],
                            2.0,
                            2.0,
                        )
                        .draw(rect, &Default::default(), ctx.transform, g);
                        Rectangle::new_round([100. / 255., 100. / 255., 100. / 255., 1.0], 2.0)
                            .draw(rect, &Default::default(), ctx.transform, g);
                        if first {
                            if let Some(&char) = self.guess.as_bytes().get(x) {
                                draw_text(
                                    &ctx,
                                    g,
                                    glyphs,
                                    [1.0; 4],
                                    Pos {
                                        x: rect[0] + SQUARE_SIZE / 4.0 + 2.0,
                                        y: rect[1] + SQUARE_SIZE / 2.0 + 5.0,
                                    },
                                    &(char as char).to_string(),
                                    30,
                                );
                            }
                        }
                    }
                    first = false;
                }
            }

            // Update glyphs before rendering
            glyphs.factory.encoder.flush(device);
        });
    }
}
