use super::{CharGuess, Game, GuessError, GuessResult, GuessType};
use crate::components::application::{MiniApp, DEFAULT_HEIGHT, DEFAULT_WIDTH};
use crate::components::button::{draw_text, Pos, UIButton};
use crate::menu::{config::Config, highscores::HighScores, ui::TOP_PAD};
use crate::Event;
use piston_window::*;

pub struct WordleApp {
    state: Game,
    guess: String,
    hover_pos: [f64; 2],
    prev_text: Option<String>,
    first_result: bool,
}

impl WordleApp {
    pub fn new() -> Self {
        WordleApp {
            state: Game::new(),
            guess: String::new(),
            hover_pos: [0.0, 0.0],
            prev_text: None,
            first_result: true,
        }
    }
}

const BOARD_SIZE: f64 = DEFAULT_HEIGHT as f64 - 100.0;
const CENTER_X: f64 = (DEFAULT_WIDTH as f64 - BOARD_SIZE) / 2.0;
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
                self.prev_text = None;
                self.state.reset();
                self.guess.clear();
                self.first_result = true;
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
                Key::Backspace | Key::Delete => {
                    self.guess.pop();
                    self.prev_text = None;
                }
                Key::Return => {
                    let result = self.state.guess(&self.guess);
                    match result {
                        Err(GuessError::GameOver(_)) => {
                            self.prev_text = Some("You ran out of tries!".to_string());

                            // update highscores
                            if self.first_result {
                                self.first_result = false;
                                highscores.scores.wordle = 0;
                                highscores.save_scores(highscores.location.clone());
                            }
                        }
                        Err(error) => {
                            self.prev_text = Some(error.to_string());
                        }
                        Ok(res) => {
                            self.prev_text = match res {
                                GuessResult::Right => {
                                    // update highscores
                                    if self.first_result {
                                        self.first_result = false;
                                        highscores.scores.wordle += 1;
                                        highscores.save_scores(highscores.location.clone());
                                    }

                                    Some("You won!".to_string())
                                }
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
                    // black
                    [1.0, 1.0, 1.0, 1.0]
                },
                Pos { x: 10.0, y: 400.0 },
                &format!("Win streak: {}", highscores.scores.wordle),
                28,
            );

            // draw win/lose/error text
            if let Some(ref text) = self.prev_text {
                if text == "You ran out of tries!" {
                    draw_text(
                        &c,
                        g,
                        glyphs,
                        [242.0 / 255.0, 87.0 / 255.0, 87.0 / 255.0, 1.0],
                        Pos { x: 10.0, y: 200.0 },
                        text,
                        24,
                    );
                    // reveal the word
                    draw_text(
                        &c,
                        g,
                        glyphs,
                        [77.0 / 255.0, 143.0 / 255.0, 69.0 / 255.0, 1.0],
                        Pos { x: 10.0, y: 225.0 },
                        format!("The word was {}", self.state.word).as_str(),
                        24,
                    );
                } else if text == "You won!" {
                    draw_text(
                        &c,
                        g,
                        glyphs,
                        [43.0 / 255.0, 1.0, 0.0, 1.0],
                        Pos { x: 10.0, y: 200.0 },
                        text,
                        20,
                    );
                } else {
                    draw_text(
                        &c,
                        g,
                        glyphs,
                        [212.0 / 255.0, 189.0 / 255.0, 59.0 / 255.0, 1.0],
                        Pos { x: 10.0, y: 200.0 },
                        text,
                        20,
                    );
                }
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
