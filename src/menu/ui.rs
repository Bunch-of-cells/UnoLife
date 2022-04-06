use piston_window::*;
use crate::{components::{application::MiniApp, button::UIButton}, tictactoe::ui::{Pos, draw_text}};

pub struct MainMenu {
    pub hover_pos: [f64; 2],
    pub tab: usize,
}

impl MainMenu {
    pub fn new() -> Self {
        MainMenu {
            hover_pos: [0.0, 0.0],
            tab: 0,
        }
    }
}

impl MiniApp for MainMenu {
    const NAME: &'static str = "Main Menu";

    fn render(&mut self, window: &mut PistonWindow, event: &Event, glyphs: &mut Glyphs) {
        if let Some([cx, cy]) = event.mouse_cursor_args() {
            self.hover_pos = [cx, cy];
        }
        // println!("x: {} y: {}", self.hover_pos[0], self.hover_pos[1]);

        // init buttons
        let mut home_button = UIButton::new(" Home", [1.0, 1.0, 1.0, 1.0], [0.0, 0.0, 0.0, 1.0], 30, Pos { x: 50.0, y: 10.0 }, 115.0, 85.0);
        let mut settings_button = UIButton::new("Settings", [1.0, 1.0, 1.0, 1.0], [0.0, 0.0, 0.0, 1.0], 30, Pos { x: 200.0, y: 10.0 }, 150.0, 85.0);

        let left_click = event.press_args() == Some(Button::Mouse(MouseButton::Left));

        // handle button events
        if home_button.is_over(self.hover_pos[0], self.hover_pos[1]) {
            if left_click {
                self.tab = 0;
            } else {
                home_button.color = [120.0 / 255.0, 120.0 / 255.0, 120.0 / 255.0, 0.75];
            }
        } else if settings_button.is_over(self.hover_pos[0], self.hover_pos[1]) {
            if left_click {
                self.tab = 1;
            } else {
                settings_button.color = [120.0 / 255.0, 120.0 / 255.0, 120.0 / 255.0, 0.75];
            }
        }

        window.draw_2d(event, |c, g, device| {
            clear([184.0 / 255.0, 183.0 / 255.0, 180.0 / 255.0, 1.0], g);

            // draw taskbar
            {
                rectangle(
                    [1.0, 1.0, 1.0, 1.0],
                    [0.0, 0.0, 1280.0, 100.0],
                    c.transform,
                    g,
                );
                // draw buttons
                home_button.draw(&c, g, glyphs);
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
                        40
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
                        40
                    );
                }
                _ => ()
            }

            // Update glyphs before rendering
            glyphs.factory.encoder.flush(device);
        });
    }
}