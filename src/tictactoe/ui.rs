use super::Board;
use crate::components::application::{MiniApp, DEFAULT_HEIGHT, DEFAULT_WIDTH};
use crate::Event;
use piston_window::*;

pub struct TicTacToeApp {
    pub state: Board,
    pub hover_pos: [f64; 2],
}

impl TicTacToeApp {
    pub fn new() -> Self {
        TicTacToeApp {
            state: Board::new(),
            hover_pos: [0.0, 0.0],
        }
    }
}

const BOARD_SIZE: f64 = DEFAULT_HEIGHT as f64 - 100.0;
const CENTER_X: f64 = (DEFAULT_WIDTH as f64 - BOARD_SIZE) / 2.0;
const TOP_PAD: f64 = 60.0;
const SQUARE_SIZE: f64 = BOARD_SIZE / 3.0;

impl MiniApp for TicTacToeApp {
    const NAME: &'static str = "TicTacToe";

    fn render(&mut self, window: &mut PistonWindow, event: &Event) {
        if let Some([cx, cy]) = event.mouse_cursor_args() {
            self.hover_pos = [cx, cy];
        }

        window.draw_2d(event, |c, g, _| {
            clear([1.0; 4], g);
            let c = c.trans(CENTER_X, TOP_PAD);

            for x in 0..3 {
                for y in 0..3 {
                    let hovered = SQUARE_SIZE * (x as f64) + CENTER_X < self.hover_pos[0]
                        && self.hover_pos[0] < SQUARE_SIZE * (x as f64 + 1.0) + CENTER_X
                        && SQUARE_SIZE * (y as f64) + TOP_PAD < self.hover_pos[1]
                        && self.hover_pos[1] < SQUARE_SIZE * (y as f64 + 1.0) + TOP_PAD;

                    let clr = match self.state.cells[y][x] {
                        super::Mark::X => [160.0 / 255.0, 237.0 / 255.0, 138.0 / 255.0, 1.0],
                        super::Mark::O => [233.0 / 255.0, 138.0 / 255.0, 237.0 / 255.0, 1.0],
                        _ => [240.0 / 255.0, 226.0 / 255.0, 168.0 / 255.0, 1.0],
                    };
                    let rect = math::margin_rectangle(
                        [
                            SQUARE_SIZE * (x as f64),
                            SQUARE_SIZE * (y as f64),
                            SQUARE_SIZE,
                            SQUARE_SIZE,
                        ],
                        if hovered { 4.0 } else { 7.0 },
                    );
                    rectangle(clr, rect, c.transform, g);
                }
            }
        });
    }
}
