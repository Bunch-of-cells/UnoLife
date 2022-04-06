extern crate piston_window;
use crate::components::application::MiniApp;
use menu::ui::MainMenu;
use tictactoe::ui::TicTacToeApp;
use piston_window::*;

mod components;
mod tictactoe;
mod wordle;
mod menu;

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

    println!("{:?}", window.size());
    let mut main_menu = MainMenu::new();
    let mut ttt = TicTacToeApp::new();

    let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets")
        .unwrap();
    let mut glyphs = window.load_font(assets.join("Roboto-Regular.ttf")).unwrap();

    let mut events = Events::new(EventSettings::new());
    while let Some(event) = events.next(&mut window) {
        main_menu.render(&mut window, &event, &mut glyphs);
    }
}
