use super::{Direction, Game};
use crate::components::application::{MiniApp, DEFAULT_WIDTH};
use crate::components::button::{draw_text, Pos};
use crate::Event;
use crate::menu::ui::TOP_PAD;
use piston_window::*;

pub struct SnakeApp {
    state: Game,
    text: Option<String>,
    dir: Option<Direction>,
}

impl SnakeApp {
    pub fn new() -> Self {
        SnakeApp {
            state: Game::new(450, 450),
            text: None,
            dir: None,
        }
    }
}

const SQUARE_SIZE: f64 = 50.0;

impl MiniApp for SnakeApp {
    fn render(&mut self, window: &mut PistonWindow, event: &Event, glyphs: &mut Glyphs) {
        self.dir = if let Some(Button::Keyboard(press)) = event.press_args() {
            match press {
                Key::Up => Some(Direction::Up),
                Key::Down => Some(Direction::Down),
                Key::Left => Some(Direction::Left),
                Key::Right => Some(Direction::Right),
                _ => None,
            }
        } else {
            None
        };

        window.draw_2d(event, |c, g, device| {
            clear([1.0; 4], g);

            if let Some(ref text) = self.text {
                draw_text(
                    &c,
                    g,
                    glyphs,
                    [0.0, 0.0, 0.0, 1.0],
                    Pos { x: 10.0, y: 528.0 },
                    text,
                    28,
                );
            }

            // Draw the board
            let ctx = c.trans(DEFAULT_WIDTH as f64, TOP_PAD);

            self.state.step(self.dir);
            if self.dir.is_some() {
                self.dir = None;
            }

            for (x, y) in (1..self.state.width).zip(1..self.state.height) {
                if self.state.snake.body.iter().any(|c| c.x == x && c.y == y) {
                    let rect = [
                        SQUARE_SIZE * (x as f64),
                        SQUARE_SIZE * (y as f64),
                        SQUARE_SIZE,
                        SQUARE_SIZE,
                    ];
                    Rectangle::new([0.0, 0.0, 0.0, 1.0]).draw(
                        rect,
                        &Default::default(),
                        ctx.transform,
                        g,
                    );
                }
            }

            // Update glyphs before rendering
            glyphs.factory.encoder.flush(device);
        });
    }
}
