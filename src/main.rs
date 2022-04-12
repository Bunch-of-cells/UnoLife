extern crate piston_window;

use std::{fs, path::Path};

use crate::components::application::MiniApp;
use menu::{config::Config, highscores::HighScores, ui::MainMenu};
use piston_window::*;
use reddit_meme::ui::UPDATE;
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

fn create_window() -> PistonWindow {
    WindowSettings::new("UnoLife", [0, 0])
        .resizable(false)
        .build::<PistonWindow>()
        .unwrap()
        .max_fps(60)
}

fn main() {
    let mut windows: Vec<_> = (0..2_usize).into_iter().map(|_| create_window()).collect();
    windows[1].hide();
    windows[0].set_size([
        components::application::DEFAULT_WIDTH,
        components::application::DEFAULT_HEIGHT,
    ]);
    windows[1].set_size([0, 0]);

    let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets")
        .unwrap();

    let file = assets.join("unolife_logo.rgba");
    let data = fs::read(file).unwrap();
    windows[0]
        .window
        .ctx
        .window()
        .set_window_icon(Icon::from_rgba(data, 500, 500).ok());

    let mut main_menu = MainMenu::new();
    let mut config = Config::fetch_config();
    let mut highscores = HighScores::fetch_scores();

    let mut glyphs = windows[0]
        .load_font(assets.join("Roboto-Regular.ttf"))
        .unwrap();
    loop {
        for i in 0..windows.len() {
            if let Some(e) = windows[i].next() {
                if i == 0 {
                    main_menu.render(&mut windows, &e, &mut glyphs, &mut config, &mut highscores);
                } else if i == 1 {
                    main_menu.apps[6].render(
                        &mut windows,
                        &e,
                        &mut glyphs,
                        &mut config,
                        &mut highscores,
                    );
                }
            }
            // if exit
            if windows[i].should_close() {
                if i == 0 {
                    std::process::exit(0);
                } else {
                    let file = if Path::new("meme.jpg").exists() {
                        "meme.jpg"
                    } else if Path::new("meme.png").exists() {
                        "meme.png"
                    } else if Path::new("meme.jpeg").exists() {
                        "meme.jpeg"
                    } else {
                        "69"
                    };
                    if file != "69" {
                        fs::remove_file(file).unwrap();
                    }
                    windows[i].hide();
                    unsafe {
                        UPDATE = false;
                    }
                }
            }
        }
    }
}
