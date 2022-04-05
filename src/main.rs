extern crate piston_window;

use std::thread::sleep;

use piston_window::*;
use rand::Rng;
use std::time::Duration;

mod components;
use components::*;

mod tictactoe;

fn main() {
    let mut rng = rand::thread_rng();
    // types::Color
    let mut window: PistonWindow =
        WindowSettings::new("Hello your computer has virus.", [640, 480])
            .build()
            .unwrap();
    while let Some(event) = window.next() {
        window.draw_2d(&event, |context, graphics, _device| {
            clear([1.0; 4], graphics);
            rectangle(
                Color::new(rng.gen::<u8>(), rng.gen::<u8>(), rng.gen::<u8>(), 1.0).as_array(),
                [0.0, 0.0, 10000.0, 10000.0],
                context.transform,
                graphics,
            );
        });
        sleep(Duration::from_millis(3));
    }
}
