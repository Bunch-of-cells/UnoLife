extern crate piston_window;
use crate::components::application::MiniApp;
// use components::{animations::bounce, button::Pos};
use menu::ui::MainMenu;
use piston_window::*;

mod components;
mod config;
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

    // let screen_resolution = window.window.ctx.window().current_monitor().unwrap().size();
    // println!("{:?}", screen_resolution);

    let mut main_menu = MainMenu::new();
    let mut config = main_menu.config.clone();

    let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets")
        .unwrap();
    let mut glyphs = window.load_font(assets.join("Roboto-Regular.ttf")).unwrap();

    let mut events = Events::new(EventSettings::new());

    // bounce(&mut window, Pos { x: 500.0, y: 100.0 }, false, 25);
    while let Some(event) = events.next(&mut window) {
        main_menu.render(&mut window, &event, &mut glyphs, &mut config);
    }
}
