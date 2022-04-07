use crate::components::{
    application::MiniApp,
    button::{draw_text, Pos, UIButton},
};
use crate::tictactoe::ui::TicTacToeApp;
use crate::wordle::ui::WordleApp;
use piston_window::*;

pub struct MainMenu {
    pub hover_pos: [f64; 2],
    pub tab: usize,
    apps: Vec<Box<dyn MiniApp>>,
}

impl MainMenu {
    pub fn new() -> Self {
        MainMenu {
            hover_pos: [0.0, 0.0],
            tab: 0,
            // make list that contains all apps
            apps: vec![
                Box::new(TicTacToeApp::new()),
                Box::new(WordleApp::new()),
            ],
        }
    }
}

impl MiniApp for MainMenu {
    fn render(&mut self, window: &mut PistonWindow, event: &Event, glyphs: &mut Glyphs) {
        if let Some([cx, cy]) = event.mouse_cursor_args() {
            self.hover_pos = [cx, cy];
        }

        // init variables
        let size = window.size();

        // init buttons
        let mut buttons = [
            UIButton::new(
                " Home",
                [1.0, 1.0, 1.0, 1.0],
                [0.0, 0.0, 0.0, 1.0],
                24,
                Pos { x: 30.0, y: 0.0 },
                102.0,
                84.0,
            ),
            UIButton::new(
                " Games",
                [1.0, 1.0, 1.0, 1.0],
                [0.0, 0.0, 0.0, 1.0],
                24,
                Pos { x: 140.0, y: 0.0 },
                115.0,
                84.0,
            ),
            UIButton::new(
                " Settings",
                [1.0, 1.0, 1.0, 1.0],
                [0.0, 0.0, 0.0, 1.0],
                24,
                Pos { x: 270.0, y: 0.0 },
                126.0,
                84.0,
            ),
            UIButton::new(
                "Play TicTacToe",
                [0.0; 4],
                [0.0, 0.0, 0.0, 1.0],
                24,
                Pos { x: 40.0, y: 120.0 },
                224.0,
                56.0,
            ),
            UIButton::new(
                "Play Wordle",
                [0.0; 4],
                [0.0, 0.0, 0.0, 1.0],
                24,
                Pos { x: 40.0, y: 180.0 },
                224.0,
                56.0,
            ),
        ];

        let left_click = event.press_args() == Some(Button::Mouse(MouseButton::Left));

        // handle button events
        for (index, button) in buttons.iter_mut().enumerate() {
            if button.is_over(self.hover_pos[0], self.hover_pos[1]) {
                if left_click {
                    self.tab = index;
                } else {
                    button.color = [120.0 / 255.0, 120.0 / 255.0, 120.0 / 255.0, 0.35];
                }
            }
        }

        match self.tab {
            0 | 1 | 2 => {
                window.draw_2d(event, |_, g, _| {
                    clear([212.0 / 255.0, 248.0 / 255.0, 1.0, 1.0], g);
                });
            }
            _ => self.apps[self.tab - 3].render(window, event, glyphs),
        };

        window.draw_2d(event, |c, g, device| {
            // draw taskbar
            {
                rectangle(
                    [1.0, 1.0, 1.0, 1.0],
                    [0.0, 0.0, size.width, 85.0],
                    c.transform,
                    g,
                );

                // draw black line sepperating the task bar from the content
                line(
                    [0.0, 0.0, 0.0, 1.0],
                    0.5,
                    [0.0, 85.0, size.width, 85.0],
                    c.transform,
                    g,
                );

                // draw buttons
                for (index, button) in buttons.iter().enumerate() {
                    if index < 3 {
                        button.draw(&c, g, glyphs);
                    }
                }
            }

            match self.tab {
                0 => {
                    // HOME TAB
                    // draw welcome text
                    draw_text(
                        &c,
                        g,
                        glyphs,
                        [0.0, 0.0, 0.0, 1.0],
                        Pos { x: 50.0, y: 300.0 },
                        "Welcome to the home tab!",
                        32,
                    );
                }
                1 => {
                    // GAMES TAB
                    // draw text
                    for (index, button) in buttons.iter().enumerate() {
                        if index > 2 {
                            button.draw(&c, g, glyphs);
                        }
                    }
                }
                2 => {
                    // SETTINGS TAB
                    // draw text
                    draw_text(
                        &c,
                        g,
                        glyphs,
                        [0.0, 0.0, 0.0, 1.0],
                        Pos { x: 50.0, y: 300.0 },
                        "Welcome to the settings tab!",
                        32,
                    );
                }
                _ => (),
            }

            // Update glyphs before rendering
            glyphs.factory.encoder.flush(device);
        });
    }
}
