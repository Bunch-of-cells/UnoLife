extern crate piston_window;

use crate::components::application::MiniApp;
use menu::{config::Config, highscores::HighScores, ui::MainMenu};
use piston_window::*;
use winit::{window::Icon, platform::windows::IconExtWindows};

mod breakout;
mod components;
mod menu;
mod puzzle15;
mod snake;
mod tictactoe;
mod twenty48;
mod wordle;

fn main() {
    let mut window = WindowSettings::new(
        "UnoLife",
        [
            components::application::DEFAULT_WIDTH,
            components::application::DEFAULT_HEIGHT,
        ],
    )
    .resizable(false)
    .build::<PistonWindow>()
    .unwrap()
    .max_fps(60);

    let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets")
        .unwrap();
    
    window.window.ctx.window().set_window_icon(
        Some(Icon::from_path(assets.join("unolife_logo.ico"), None).unwrap())
    );
    
    let mut main_menu = MainMenu::new();
    let mut config = Config::fetch_config();
    let mut highscores = HighScores::fetch_scores();

    let mut glyphs = window.load_font(assets.join("Roboto-Regular.ttf")).unwrap();

    while let Some(event) = window.next() {
        main_menu.render(
            &mut window,
            &event,
            &mut glyphs,
            &mut config,
            &mut highscores,
        );
    }
}
