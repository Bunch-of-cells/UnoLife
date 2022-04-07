use crate::tictactoe::ui::TicTacToeApp;
use crate::{
    components::{application::MiniApp, button::UIButton},
    tictactoe::ui::{draw_text, Pos},
};
use piston_window::*;

pub struct MainMenu {
    pub hover_pos: [f64; 2],
    pub tab: usize,
    ttt_app: TicTacToeApp,
}

impl MainMenu {
    pub fn new() -> Self {
        MainMenu {
            hover_pos: [0.0, 0.0],
            tab: 0,
            ttt_app: TicTacToeApp::new(),
        }
    }
}

impl MiniApp for MainMenu {
    const NAME: &'static str = "Main Menu";

    fn render(&mut self, window: &mut PistonWindow, event: &Event, glyphs: &mut Glyphs) {
        if let Some([cx, cy]) = event.mouse_cursor_args() {
            self.hover_pos = [cx, cy];
        }

        // init variables
        let size = window.size();

        // init buttons
        let mut home_button = UIButton::new(
            " Home",
            [1.0, 1.0, 1.0, 1.0],
            [0.0, 0.0, 0.0, 1.0],
            24,
            Pos { x: 30.0, y: 0.0 },
            102.0,
            85.0,
        );
        let mut games_button = UIButton::new(
            " Games",
            [1.0, 1.0, 1.0, 1.0],
            [0.0, 0.0, 0.0, 1.0],
            24,
            Pos { x: 140.0, y: 0.0 },
            115.0,
            85.0,
        );
        let mut settings_button = UIButton::new(
            " Settings",
            [1.0, 1.0, 1.0, 1.0],
            [0.0, 0.0, 0.0, 1.0],
            24,
            Pos { x: 270.0, y: 0.0 },
            126.0,
            85.0,
        );

        let mut ttt_button = UIButton::new(
            "Play TicTacToe",
            [0.0; 4],
            [0.0, 0.0, 0.0, 1.0],
            24,
            Pos { x: 40.0, y: 120.0 },
            224.0,
            56.0,
        );

        let left_click = event.press_args() == Some(Button::Mouse(MouseButton::Left));

        // handle button events
        if home_button.is_over(self.hover_pos[0], self.hover_pos[1]) {
            if left_click {
                self.tab = 0;
            } else {
                home_button.color = [120.0 / 255.0, 120.0 / 255.0, 120.0 / 255.0, 0.35];
            }
        } else if settings_button.is_over(self.hover_pos[0], self.hover_pos[1]) {
            if left_click {
                self.tab = 1;
            } else {
                settings_button.color = [120.0 / 255.0, 120.0 / 255.0, 120.0 / 255.0, 0.35];
            }
        } else if games_button.is_over(self.hover_pos[0], self.hover_pos[1]) {
            if left_click {
                self.tab = 2;
            } else {
                games_button.color = [120.0 / 255.0, 120.0 / 255.0, 120.0 / 255.0, 0.35];
            }
        } else if self.tab == 2 && ttt_button.is_over(self.hover_pos[0], self.hover_pos[1]) {
            if left_click {
                self.tab = 3;
            } else {
                ttt_button.color = [120.0 / 255.0, 120.0 / 255.0, 120.0 / 255.0, 0.35];
            }
        }

        match self.tab {
            0 | 1 | 2 => {
                window.draw_2d(event, |_, g, _| {
                    clear([212.0 / 255.0, 248.0 / 255.0, 1.0, 1.0], g);
                });
            }
            3 => {
                self.ttt_app.render(window, event, glyphs);
            }
            _ => (),
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
                home_button.draw(&c, g, glyphs);
                games_button.draw(&c, g, glyphs);
                settings_button.draw(&c, g, glyphs);
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
                2 => {
                    // GAMES TAB
                    // draw text
                    ttt_button.draw(&c, g, glyphs);
                }
                _ => (),
            }

            // Update glyphs before rendering
            glyphs.factory.encoder.flush(device);
        });
    }
}
