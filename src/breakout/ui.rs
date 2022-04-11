use super::{
    Game, HorizontalMovement, BALL_SIZE, BOTTOM_WALL, BRICK_SIZE, LEFT_WALL, PADDLE_SIZE,
    RIGHT_WALL, TOP_WALL,
};
use crate::components::application::{MiniApp, DEFAULT_HEIGHT, DEFAULT_WIDTH};
use crate::components::{
    button::{draw_text, Pos, UIButton},
    color::Color,
};
use crate::menu::{config::Config, highscores::HighScores, ui::TOP_PAD};
use crate::{rgb, Event};
use piston_window::*;

pub struct BreakoutApp {
    state: Game,
    hover_pos: [f64; 2],
}

impl BreakoutApp {
    pub fn new() -> Self {
        BreakoutApp {
            state: Game::new(),
            hover_pos: [0.0, 0.0],
        }
    }
}

const BOARD_SIZE: f64 = DEFAULT_HEIGHT as f64 - 100.0;
const CENTER_X: f64 = (DEFAULT_WIDTH as f64 - BOARD_SIZE) / 2.0;

impl MiniApp for BreakoutApp {
    fn render(
        &mut self,
        window: &mut PistonWindow,
        event: &Event,
        glyphs: &mut Glyphs,
        config: &mut Config,
        highscores: &mut HighScores,
    ) {
        if let Some([cx, cy]) = event.mouse_cursor_args() {
            self.hover_pos = [cx, cy];
        }

        // init buttons
        let mut reset_button = UIButton::new(
            "     Reset",
            Color::RESET,
            Color::WHITE,
            24,
            Pos { x: 791.2, y: 135.2 },
            160.0,
            48.0,
        );

        let left_click = event.press_args() == Some(Button::Mouse(MouseButton::Left));

        // handle button events
        if reset_button.is_over(self.hover_pos[0], self.hover_pos[1]) {
            if left_click {
                self.state.reset();
            } else {
                reset_button.width += 6.0;
                reset_button.pos.x -= 3.0;
                reset_button.height += 6.0;
                reset_button.pos.y -= 3.0;
                reset_button.size += 1;
            }
        }

        if let Some(Button::Keyboard(press)) = event.press_args() {
            match press {
                Key::Left => {
                    self.state.update(Some(HorizontalMovement::Left));
                }
                Key::Right => {
                    self.state.update(Some(HorizontalMovement::Right));
                }
                _ => (),
            }
        }

        window.draw_2d(event, |c, g, device| {
            clear(
                if config.options.white_theme {
                    Color::WHITE
                } else {
                    Color::DARK_THEME_BG
                },
                g,
            );

            // draw buttons
            reset_button.draw(&c, g, glyphs);

            // draw highscores
            draw_text(
                &c,
                g,
                glyphs,
                if config.options.white_theme {
                    Color::BLACK
                } else {
                    Color::WHITE
                },
                Pos { x: 10.0, y: 400.0 },
                &format!("Win streak: {}", highscores.scores.wordle),
                28,
            );

            let ctx = c.trans(0.0, TOP_PAD);

            // Draw Walls
            Rectangle::new_border(Color::BLACK, 1.0).draw(
                [
                    LEFT_WALL as f64,
                    TOP_WALL as f64,
                    (RIGHT_WALL - LEFT_WALL) as f64,
                    (BOTTOM_WALL - TOP_WALL) as f64,
                ],
                &Default::default(),
                ctx.transform,
                g,
            );

            // Draw bricks
            for brick in &self.state.bricks {
                let rect = [
                    (brick.x + BRICK_SIZE[0]) as f64,
                    (brick.y + BRICK_SIZE[1]) as f64,
                    BRICK_SIZE[0] as f64,
                    BRICK_SIZE[1] as f64,
                ];
                rectangle(rgb!(200, 200, 100), rect, ctx.transform, g);
            }

            // Draw paddle
            let paddle_rect = [
                (self.state.paddle.x + PADDLE_SIZE[0]) as f64,
                (self.state.paddle.y + PADDLE_SIZE[1]) as f64,
                PADDLE_SIZE[0] as f64,
                PADDLE_SIZE[1] as f64,
            ];
            rectangle(rgb!(150, 150, 150), paddle_rect, ctx.transform, g);

            // Draw ball
            let ball_rect = [
                (self.state.ball.x + BALL_SIZE) as f64,
                (self.state.ball.y + BALL_SIZE) as f64,
                self.state.ball.x as f64,
                self.state.ball.y as f64,
            ];
            rectangle(rgb!(100, 200, 100), ball_rect, ctx.transform, g);

            // Run
            self.state.update(None);

            // Update glyphs before rendering
            glyphs.factory.encoder.flush(device);
        });
    }
}
