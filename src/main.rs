extern crate piston_window;
use std::thread::sleep;

use crate::components::application::MiniApp;
// use components::{animations::bounce, button::Pos};
use menu::{ui::MainMenu, config::Config};
use piston_window::*;

mod components;
mod menu;
mod snake;
mod tictactoe;
mod wordle;

fn main() {
    let mut window: PistonWindow = WindowSettings::new(
        "UnoLife",
        [
            components::application::DEFAULT_WIDTH,
            components::application::DEFAULT_HEIGHT,
        ],
    )
    .resizable(false)
    .build()
    .unwrap();

    window.set_lazy(true);

    let mut main_menu = MainMenu::new();
    let mut config = Config::fetch_config();

    let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets")
        .unwrap();
    let mut glyphs = window.load_font(assets.join("Roboto-Regular.ttf")).unwrap();

    // bounce(&mut window, Pos { x: 500.0, y: 100.0 }, false, 25);
    while let Some(event) = window.next() {
        main_menu.render(&mut window, &event, &mut glyphs, &mut config);
        sleep(std::time::Duration::from_millis(1));
    }
}
