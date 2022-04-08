extern crate piston_window;
use crate::components::application::MiniApp;
use menu::ui::MainMenu;
use piston_window::*;

mod components;
mod menu;
mod tictactoe;
mod wordle;

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

    // let screen_resolution = window.window.ctx.window().current_monitor().unwrap().size();
    // println!("{:?}", screen_resolution);

    let mut main_menu = MainMenu::new();

    let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets")
        .unwrap();
    let mut glyphs = window.load_font(assets.join("Roboto-Regular.ttf")).unwrap();

    let mut events = Events::new(EventSettings::new());
    // let mut iters: usize = 0;
    while let Some(event) = events.next(&mut window) {
        main_menu.render(&mut window, &event, &mut glyphs);
        // let t = components::animations::sin_bounce(500.0, 1.0, iters as f64);
        // if let Some(g) = t {
        //     window.set_position((g as i32, window.get_position().unwrap().y));
        // }
        // iters = (iters + 1).min(10000000);
    }
}
