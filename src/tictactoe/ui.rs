use super::Board;
use crate::components::application::{MiniApp, DEFAULT_HEIGHT, DEFAULT_WIDTH};
use crate::components::button::*;
use crate::tictactoe::{negamax_root, Mark};
use crate::Event;
use piston_window::*;

pub struct TicTacToeApp {
    pub state: Board,
    pub hover_pos: [f64; 2],
    pub hover_sq: (usize, usize),
    pub playing_ai: u8,
}

impl TicTacToeApp {
    pub fn new() -> Self {
        TicTacToeApp {
            state: Board::new(),
            hover_pos: [0.0, 0.0],
            hover_sq: (6, 6),
            playing_ai: 2,
        }
    }
}

const BOARD_SIZE: f64 = DEFAULT_HEIGHT as f64 - 100.0;
const CENTER_X: f64 = (DEFAULT_WIDTH as f64 - BOARD_SIZE) / 2.0;
const TOP_PAD: f64 = 104.0;
const SQUARE_SIZE: f64 = BOARD_SIZE / 4.0;

// Converts Mark to Color
fn mark_to_clr(mark: super::Mark) -> [f32; 4] {
    match mark {
        super::Mark::X => [160.0 / 255.0, 237.0 / 255.0, 138.0 / 255.0, 1.0],
        super::Mark::O => [233.0 / 255.0, 138.0 / 255.0, 237.0 / 255.0, 1.0],
        _ => [250.0 / 255.0, 246.0 / 255.0, 188.0 / 255.0, 1.0],
    }
}

impl MiniApp for TicTacToeApp {
    const NAME: &'static str = "TicTacToe";

    fn render(&mut self, window: &mut PistonWindow, event: &Event, glyphs: &mut Glyphs) {
        if let Some([cx, cy]) = event.mouse_cursor_args() {
            self.hover_pos = [cx, cy];
        }

        let left_click = event.press_args() == Some(Button::Mouse(MouseButton::Left));

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

        let ai_text = if self.playing_ai == 1 {
            " Mode: Purple vs AI"
        } else if self.playing_ai == 2 {
            "  Mode: Lime vs AI"
        } else {
            "  Mode: Man vs Man"
        };
        let mut ai_button = UIButton::new(
            ai_text,
            [115.0 / 255.0, 115.0 / 255.0, 115.0 / 255.0, 1.0],
            [1.0, 1.0, 1.0, 1.0],
            14,
            Pos { x: 791.2, y: 228.0 },
            160.0,
            48.0,
        );

        // handle button events
        if reset_button.is_over(self.hover_pos[0], self.hover_pos[1]) {
            if left_click {
                self.state.reset();
                if self.playing_ai == (self.state.turn as u8 + 1) {
                    let move_ = negamax_root(&mut self.state);
                    self.state.make_move(move_.0, move_.1);
                }
            } else {
                reset_button.width += 6.0;
                reset_button.pos.x -= 3.0;
                reset_button.height += 6.0;
                reset_button.pos.y -= 3.0;
                reset_button.size += 1;
            }
        } else if ai_button.is_over(self.hover_pos[0], self.hover_pos[1]) {
            if left_click {
                if self.playing_ai != 2 {
                    self.playing_ai += 1;
                } else {
                    self.playing_ai = 0;
                }
            } else {
                ai_button.width += 6.0;
                ai_button.pos.x -= 3.0;
                ai_button.height += 6.0;
                ai_button.pos.y -= 3.0;
                ai_button.size += 1;
            }
        }

        self.hover_sq = (6, 6);

        'o: for x in 0..3 {
            for y in 0..3 {
                let hovered = SQUARE_SIZE * (x as f64) + CENTER_X + 100.0 < self.hover_pos[0]
                    && self.hover_pos[0] < SQUARE_SIZE * (x as f64 + 1.0) + CENTER_X + 100.0
                    && SQUARE_SIZE * (y as f64) + TOP_PAD < self.hover_pos[1]
                    && self.hover_pos[1] < SQUARE_SIZE * (y as f64 + 1.0) + TOP_PAD;

                if hovered {
                    self.hover_sq = (x, y);

                    if left_click {
                        let is_free = self.state.cells[y][x] == Mark::None;
                        if is_free {
                            self.state.make_move(y, x);
                            // if playing against AI, make a move
                            if self.playing_ai == (self.state.turn as u8 + 1) {
                                let move_ = negamax_root(&mut self.state);
                                self.state.make_move(move_.0, move_.1);
                            }
                        }
                    }

                    break 'o;
                }
            }
        }

        window.draw_2d(event, |c, g, device| {
            clear([1.0; 4], g);

            // Draw texts
            let result = self.state.is_over();
            if result == Mark::X {
                draw_text(
                    &c,
                    g,
                    glyphs,
                    [0.0, 0.0, 0.0, 1.0],
                    Pos { x: 450.0, y: 528.0 },
                    "Lime wins!",
                    32,
                );
            } else if result == Mark::O {
                draw_text(
                    &c,
                    g,
                    glyphs,
                    [0.0, 0.0, 0.0, 1.0],
                    Pos { x: 440.0, y: 528.0 },
                    "Purple wins!",
                    32,
                );
            } else if self.state.is_draw() {
                draw_text(
                    &c,
                    g,
                    glyphs,
                    [0.0, 0.0, 0.0, 1.0],
                    Pos { x: 442.0, y: 528.0 },
                    "It's a draw!",
                    32,
                );
            }

            // Draw buttons
            reset_button.draw(&c, g, glyphs);
            ai_button.draw(&c, g, glyphs);

            {
                // Draw the STM
                let ctx = c.trans(120.0, TOP_PAD * 2.0);
                let rect =
                    math::margin_rectangle([0.0, 0.0, SQUARE_SIZE / 1.4, SQUARE_SIZE / 1.4], 0.0);
                rectangle(mark_to_clr(self.state.turn), rect, ctx.transform, g);
            }

            {
                // Draw the board
                let ctx = c.trans(CENTER_X + 80.0, TOP_PAD);

                for x in 0..3 {
                    for y in 0..3 {
                        let hovered = self.hover_sq == (x, y);

                        let clr = mark_to_clr(self.state.cells[y][x]);
                        let rect = math::margin_rectangle(
                            [
                                SQUARE_SIZE * (x as f64),
                                SQUARE_SIZE * (y as f64),
                                SQUARE_SIZE,
                                SQUARE_SIZE,
                            ],
                            if hovered { 4.0 } else { 7.0 },
                        );
                        rectangle(clr, rect, ctx.transform, g);
                    }
                }
            }

            // Update glyphs before rendering
            glyphs.factory.encoder.flush(device);
        });
    }
}
