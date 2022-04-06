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
    ttt_app.state.cells = [
        [
            tictactoe::Mark::O,
            tictactoe::Mark::X,
            tictactoe::Mark::None,
        ],
        [
            tictactoe::Mark::None,
            tictactoe::Mark::O,
            tictactoe::Mark::None,
        ],
        [
            tictactoe::Mark::None,
            tictactoe::Mark::X,
            tictactoe::Mark::X,
        ],
    ];

    let mut events = Events::new(EventSettings::new());
    while let Some(event) = events.next(&mut window) {
        ttt_app.render(&mut window, &event);
    }
}
