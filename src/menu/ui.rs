use crate::puzzle15::ui::Puzzle15App;
use crate::snake::ui::SnakeApp;
use crate::tictactoe::ui::TicTacToeApp;
use crate::twenty48::ui::Twenty48App;
use crate::wordle::ui::WordleApp;
use crate::{
    components::{
        application::MiniApp,
        button::{draw_text, Pos, UIButton},
        color::Color,
    },
    rgb,
};

use piston_window::*;

use super::{config::Config, highscores::HighScores};

pub const TOP_PAD: f64 = 104.0;

const GAMES: usize = 5;
pub struct MainMenu {
    pub hover_pos: [f64; 2],
    pub tab: usize,
    apps: [Box<dyn MiniApp>; GAMES],
}

impl MainMenu {
    pub fn new() -> Self {
        MainMenu {
            hover_pos: [0.0, 0.0],
            tab: 0,
            // make list that contains all apps
            apps: [
                Box::new(TicTacToeApp::new()),
                Box::new(WordleApp::new()),
                Box::new(SnakeApp::new()),
                Box::new(Twenty48App::new()),
                Box::new(Puzzle15App::new()),
            ],
        }
    }
}

impl MiniApp for MainMenu {
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

        // init variables
        let size = window.size();

        // init buttons
        let mut buttons = [
            UIButton::new(
                " Home",
                Color::CLEAR,
                Color::BLACK,
                24,
                Pos { x: 30.0, y: 0.0 },
                102.0,
                84.0,
            ),
            UIButton::new(
                " Games",
                Color::CLEAR,
                Color::BLACK,
                24,
                Pos { x: 140.0, y: 0.0 },
                115.0,
                84.0,
            ),
            UIButton::new(
                " Settings",
                Color::CLEAR,
                Color::BLACK,
                24,
                Pos { x: 270.0, y: 0.0 },
                126.0,
                84.0,
            ),
            UIButton::new(
                "Play TicTacToe",
                Color::CLEAR,
                Color::BLACK,
                24,
                Pos { x: 40.0, y: 120.0 },
                224.0,
                56.0,
            ),
            UIButton::new(
                "Play Wordle",
                Color::CLEAR,
                Color::BLACK,
                24,
                Pos { x: 40.0, y: 180.0 },
                224.0,
                56.0,
            ),
            UIButton::new(
                "Play Snake",
                Color::CLEAR,
                Color::BLACK,
                24,
                Pos { x: 40.0, y: 240.0 },
                224.0,
                56.0,
            ),
            UIButton::new(
                "Play 2048",
                Color::CLEAR,
                Color::BLACK,
                24,
                Pos { x: 40.0, y: 300.0 },
                224.0,
                56.0,
            ),
            UIButton::new(
                "Play 15 Puzzle",
                Color::CLEAR,
                Color::BLACK,
                24,
                Pos { x: 40.0, y: 360.0 },
                224.0,
                56.0,
            ),
        ];

        let mut config_buttons = [
            UIButton::new(
                "Dark Theme",
                Color::CLEAR,
                Color::BLACK,
                24,
                Pos { x: 40.0, y: 120.0 },
                224.0,
                56.0,
            ),
            UIButton::new(
                "Reset Highscores",
                Color::RESET,
                Color::WHITE,
                24,
                Pos { x: 50.0, y: 200.0 },
                240.0,
                56.0,
            ),
        ];

        // change style's depending on theme
        if !config.options.white_theme {
            config_buttons[0].text = "Light Theme".to_string();
            config_buttons[0].text_color = Color::WHITE;

            for button in buttons.iter_mut() {
                button.text_color = Color::WHITE;
            }
        }

        let left_click = event.press_args() == Some(Button::Mouse(MouseButton::Left));

        // handle button events
        for (index, button) in buttons.iter_mut().enumerate() {
            if (index < 3 || self.tab == 1) && button.is_over(self.hover_pos[0], self.hover_pos[1])
            {
                if left_click {
                    self.tab = index;
                } else {
                    button.color = rgb!(120, 120, 120, 0.35);
                }
            }
        }

        // handle config button events
        for (index, button) in config_buttons.iter_mut().enumerate() {
            if self.tab == 2 && button.is_over(self.hover_pos[0], self.hover_pos[1]) {
                if left_click {
                    match index {
                        0 => {
                            config.options.white_theme = !config.options.white_theme;
                            config.save_config(config.location.clone());
                        }
                        1 => {
                            highscores.reset_highscores();
                            highscores.save_scores();
                        }
                        _ => (),
                    }
                } else {
                    match index {
                        1 => {
                            button.width += 6.0;
                            button.pos.x -= 3.0;
                            button.height += 6.0;
                            button.pos.y -= 3.0;
                            button.size += 1;
                        }
                        _ => button.color = rgb!(120, 120, 120, 0.35),
                    }
                }
            }
        }

        match self.tab {
            0 | 1 | 2 => {
                window.draw_2d(event, |_, g, _| {
                    clear(
                        if config.options.white_theme {
                            rgb!(212, 248, 255)
                        } else {
                            rgb!(30, 30, 30)
                        },
                        g,
                    );
                });
            }
            _ => self.apps[self.tab - 3].render(window, event, glyphs, config, highscores),
        };

        window.draw_2d(event, |c, g, device| {
            // draw taskbar
            {
                rectangle(
                    if config.options.white_theme {
                        Color::WHITE
                    } else {
                        rgb!(60, 60, 60)
                    },
                    [0.0, 0.0, size.width, 85.0],
                    c.transform,
                    g,
                );

                // draw black line sepperating the task bar from the content
                line(
                    Color::BLACK,
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
                        if config.options.white_theme {
                            Color::BLACK
                        } else {
                            Color::WHITE
                        },
                        Pos { x: 50.0, y: 300.0 },
                        "Welcome to the home tab!",
                        32,
                    );
                }
                1 => {
                    // GAMES TAB
                    // draw
                    for (index, button) in buttons.iter().enumerate() {
                        if index > 2 {
                            button.draw(&c, g, glyphs);
                        }
                    }
                }
                2 => {
                    // SETTINGS TAB
                    // draw
                    for button in config_buttons {
                        button.draw(&c, g, glyphs);
                    }
                }
                _ => (),
            }

            // Update glyphs before rendering
            glyphs.factory.encoder.flush(device);
        });
    }
}
