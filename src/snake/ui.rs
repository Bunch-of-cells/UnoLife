use super::{Direction, Game, GameState};
use crate::components::application::MiniApp;
use crate::components::button::{draw_text, Pos};
use crate::menu::{config::Config, highscores::HighScores, ui::TOP_PAD};
use crate::Event;
use piston_window::*;

pub struct SnakeApp {
    game: Game,
    state: Option<String>,
    dir: Option<Direction>,
}

impl SnakeApp {
    pub fn new() -> Self {
        SnakeApp {
            game: Game::new(450, 450),
            state: None,
            dir: None,
        }
    }
}

const SQUARE_SIZE: f64 = 10.0;

impl MiniApp for SnakeApp {
    fn render(
        &mut self,
        window: &mut PistonWindow,
        event: &Event,
        glyphs: &mut Glyphs,
        _config: &mut Config,
        _highscores: &mut HighScores,
    ) {
        self.dir = if let Some(Button::Keyboard(press)) = event.press_args() {
            match press {
                Key::Up => Some(Direction::Up),
                Key::Down => Some(Direction::Down),
                Key::Left => Some(Direction::Left),
                Key::Right => Some(Direction::Right),
                _ => None,
            }
        } else {
            self.dir
        };

        window.draw_2d(event, |c, g, device| {
            clear([1.0; 4], g);

            if let Some(ref text) = self.state {
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

            let ctx = c.trans(0.0, TOP_PAD);

            match self.game.state {
                GameState::Playing => {
                    self.game.step(self.dir.take());
                }
                GameState::Lost => todo!(),
                GameState::Won => todo!(),
            }

            for (x, y) in
                (1..self.game.width).flat_map(|x| (1..self.game.height).map(move |y| (x, y)))
            {
                if self.game.snake.body.iter().any(|c| c.x == x && c.y == y) {
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
