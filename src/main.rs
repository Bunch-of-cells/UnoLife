extern crate piston_window;

use std::{fs, path::PathBuf};

use crate::components::application::MiniApp;
use lazy_static::lazy_static;
use menu::{config::Config, highscores::HighScores, ui::MainMenu};
use piston_window::*;
use winit::window::Icon;

mod breakout;
mod components;
mod menu;
mod puzzle15;
mod reddit_meme;
mod snake;
mod tictactoe;
mod twenty48;
mod wordle;

lazy_static! {
    pub static ref ASSETS: PathBuf = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets")
        .unwrap();
}

fn create_window(dim: [u32; 2]) -> PistonWindow {
    WindowSettings::new("UnoLife", dim)
        .resizable(false)
        .build::<PistonWindow>()
        .unwrap()
        .max_fps(60)
}

fn main() {
    let mut window = create_window([
        components::application::DEFAULT_WIDTH,
        components::application::DEFAULT_HEIGHT,
    ]);

    // Set the Icon
    let file = ASSETS.join("unolife_logo.rgba");
    let data = fs::read(file).unwrap();
    window
        .window
        .ctx
        .window()
        .set_window_icon(Icon::from_rgba(data, 500, 500).ok());

    // Initialize main menu
    let mut main_menu = MainMenu::new();
    let mut config = Config::fetch_config();
    let mut highscores = HighScores::fetch_scores();

    // Load font
    let mut glyphs = window.load_font(ASSETS.join("Roboto-Regular.ttf")).unwrap();

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
