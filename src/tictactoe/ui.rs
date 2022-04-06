use super::Board;
use crate::components::application::{MiniApp, DEFAULT_HEIGHT, DEFAULT_WIDTH};
use crate::Event;
use piston_window::*;
use std::cmp::min;

pub struct TicTacToeApp {
    pub state: Board,
}

impl TicTacToeApp {
    pub fn new() -> Self {
        TicTacToeApp {
            state: Board::new(),
        }
    }
}

const BOARD_SIZE: f64 = DEFAULT_HEIGHT as f64 - 100.0;
const CENTER_X: f64 = (DEFAULT_WIDTH as f64 - BOARD_SIZE) / 2.0;
const SQUARE_SIZE: f64 = BOARD_SIZE / 3.0;

impl MiniApp for TicTacToeApp {
    const NAME: &'static str = "TicTacToe";

    fn render(&mut self, window: &mut PistonWindow, event: &Event) {
        window.draw_2d(event, |c, g, _| {
            clear([1.0; 4], g);
            let c = c.trans(CENTER_X, 50.0);
            let clr = [240.0 / 255.0, 226.0 / 255.0, 168.0 / 255.0, 1.0];
            for x in 0..3 {
                for y in 0..3 {
                    let rect = math::margin_rectangle(
                        [
                            SQUARE_SIZE * (x as f64),
                            SQUARE_SIZE * (y as f64),
                            SQUARE_SIZE,
                            SQUARE_SIZE,
                        ],
                        5.0,
                    );
                    rectangle(clr, rect, c.transform, g);
                }
            }
        });
    }
}
