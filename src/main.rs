extern crate piston_window;

use crate::components::application::MiniApp;
use piston_window::*;

mod components;
mod tictactoe;

use crate::tictactoe::ui::TicTacToeApp;

fn main() {
    let mut window: PistonWindow = WindowSettings::new(
        "MiniGame Collection",
        [
            components::application::DEFAULT_WIDTH,
            components::application::DEFAULT_HEIGHT,
        ],
    )
    .resizable(false)
    .build()
    .unwrap();
    let mut ttt_app = TicTacToeApp::new();

    let mut events = Events::new(EventSettings::new());
    while let Some(event) = events.next(&mut window) {
        ttt_app.render(&mut window, &event);
    }
}
