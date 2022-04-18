// use crate::breakout::ui::BreakoutApp;
use crate::puzzle15::ui::Puzzle15App;
use crate::reddit_meme::ui::{MemeApp, UPDATE};
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

use super::{config::Config, highscores::HighScores};
use piston_window::*;

pub const TOP_PAD: f64 = 104.0;
pub const TASKBAR_HEIGHT: f64 = 85.0;

const GAMES: usize = 6;
pub struct MainMenu {
    pub hover_pos: [f64; 2],
    pub tab: usize,
    pub apps: [Box<dyn MiniApp>; GAMES],
    prev_tab: usize,
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
                Box::new(MemeApp::new()),
                // Box::new(BreakoutApp::new()),
            ],
            prev_tab: 69,
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
        let mut tabs = [
            UIButton::new(
                " Home",
                Color::CLEAR,
                Color::BLACK,
                24,
                Pos { x: 30.0, y: 0.0 },
                100.0,
                84.0,
            ),
            UIButton::new(
                " Games",
                Color::CLEAR,
                Color::BLACK,
                24,
                Pos { x: 130.0, y: 0.0 },
                115.0,
                84.0,
            ),
            UIButton::new(
                " Settings",
                Color::CLEAR,
                Color::BLACK,
                24,
                Pos { x: 245.0, y: 0.0 },
                120.0,
                84.0,
            ),
            UIButton::new(
                "About Us",
                Color::CLEAR,
                Color::BLACK,
                24,
                Pos { x: 370.0, y: 0.0 },
                120.0,
                84.0,
            ),
        ];

        let mut game_buttons = [
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
            UIButton::new(
                "Show Meme",
                Color::CLEAR,
                Color::BLACK,
                24,
                Pos { x: 40.0, y: 420.0 },
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

            for button in tabs.iter_mut() {
                button.text_color = Color::WHITE;
            }

            for button in game_buttons.iter_mut() {
                button.text_color = Color::WHITE;
            }
        }

        let left_click = event.press_args() == Some(Button::Mouse(MouseButton::Left));

        // handle button events
        for (index, button) in tabs.iter_mut().enumerate() {
            if button.is_over(self.hover_pos[0], self.hover_pos[1]) {
                if left_click {
                    self.tab = index;
                    self.prev_tab = 69;
                } else {
                    button.color = rgb!(120, 120, 120, 0.35);
                }
            }
        }

        // Handle if hovered on / pressed buttons in game tab
        if self.tab == 1 {
            for (index, button) in game_buttons.iter_mut().enumerate() {
                if button.is_over(self.hover_pos[0], self.hover_pos[1]) {
                    if left_click {
                        self.tab = index + tabs.len();
                        self.prev_tab = 69;
                    } else {
                        button.color = rgb!(120, 120, 120, 0.35);
                    }
                }
            }
        }

        // handle config button events
        if self.tab == 2 {
            for (index, button) in config_buttons.iter_mut().enumerate() {
                if button.is_over(self.hover_pos[0], self.hover_pos[1]) {
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
        }

        match self.tab {
            tab if tab < tabs.len() => {
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

                // set update to true because tab is not meme app
                unsafe {
                    UPDATE = true;
                }

                // set window title
                if self.prev_tab != self.tab {
                    window.set_title(format!("UnoLife - {}", tabs[self.tab].text.clone().trim()));
                    self.prev_tab = self.tab;
                }
            }
            _ => {
                self.apps[self.tab - tabs.len()].render(window, event, glyphs, config, highscores);

                // set window title
                if self.prev_tab != self.tab && self.tab != 8 {
                    window.set_title(format!(
                        "UnoLife - {}",
                        game_buttons[self.tab - tabs.len()].text.clone().trim()
                    ));
                    self.prev_tab = self.tab;
                }
            }
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
                for button in tabs.iter() {
                    button.draw(&c, g, glyphs);
                }
            }

            match self.tab {
                0 => {
                    // HOME TAB
                    // draw text
                    draw_text(
                        &c,
                        g,
                        glyphs,
                        if config.options.white_theme {
                            Color::BLACK
                        } else {
                            Color::WHITE
                        },
                        Pos {
                            x: 50.0,
                            y: TASKBAR_HEIGHT + 80.0,
                        },
                        "Welcome to UnoLife!",
                        30,
                    );
                    draw_text(
                        &c,
                        g,
                        glyphs,
                        if config.options.white_theme {
                            Color::BLACK
                        } else {
                            Color::WHITE
                        },
                        Pos {
                            x: 50.0,
                            y: TASKBAR_HEIGHT + 170.0,
                        },
                        "Click on the games tab to start playing dozens of games!",
                        24,
                    );
                    draw_text(
                        &c,
                        g,
                        glyphs,
                        if config.options.white_theme {
                            Color::BLACK
                        } else {
                            Color::WHITE
                        },
                        Pos {
                            x: 50.0,
                            y: TASKBAR_HEIGHT + 205.0,
                        },
                        "Click on the settings tab to change the theme and other settings!",
                        24,
                    );
                    draw_text(
                        &c,
                        g,
                        glyphs,
                        if config.options.white_theme {
                            Color::BLACK
                        } else {
                            Color::WHITE
                        },
                        Pos {
                            x: 50.0,
                            y: TASKBAR_HEIGHT + 240.0,
                        },
                        "Check out the about us tab for more information on the creators!",
                        24,
                    );
                    draw_text(
                        &c,
                        g,
                        glyphs,
                        if config.options.white_theme {
                            Color::BLACK
                        } else {
                            Color::WHITE
                        },
                        Pos {
                            x: 50.0,
                            y: TASKBAR_HEIGHT + 330.0,
                        },
                        "And most importantly, have fun!",
                        24,
                    );
                }
                1 => {
                    // GAMES TAB
                    // draw
                    for button in game_buttons.iter() {
                        button.draw(&c, g, glyphs);
                    }
                }
                2 => {
                    // SETTINGS TAB
                    // draw
                    for button in config_buttons {
                        button.draw(&c, g, glyphs);
                    }
                }
                3 => {
                    // ABOUS US TAB
                    // draw
                    draw_text(
                        &c,
                        g,
                        glyphs,
                        if config.options.white_theme {
                            Color::BLACK
                        } else {
                            Color::WHITE
                        },
                        Pos {
                            x: 50.0,
                            y: TASKBAR_HEIGHT + 70.0,
                        },
                        "FirePlank",
                        30,
                    );

                    draw_text(
                        &c,
                        g,
                        glyphs,
                        if config.options.white_theme {
                            Color::BLACK
                        } else {
                            Color::WHITE
                        },
                        Pos {
                            x: 50.0,
                            y: TASKBAR_HEIGHT + 125.0,
                        },
                        "Hi! I'm a 16 yr old programmer, hacker, pentester and a chess enthusiast.",
                        20,
                    );
                    draw_text(
                        &c,
                        g,
                        glyphs,
                        if config.options.white_theme {
                            Color::BLACK
                        } else {
                            Color::WHITE
                        },
                        Pos {
                            x: 50.0,
                            y: TASKBAR_HEIGHT + 155.0,
                        },
                        "We made this app to test out GUI development in rust and learn more about it.",
                        20,
                    );
                    draw_text(
                        &c,
                        g,
                        glyphs,
                        if config.options.white_theme {
                            Color::BLACK
                        } else {
                            Color::WHITE
                        },
                        Pos {
                            x: 50.0,
                            y: TASKBAR_HEIGHT + 185.0,
                        },
                        "You can find my github as one of the contributors on the github page of UnoLife.",
                        20,
                    );
                    draw_text(
                        &c,
                        g,
                        glyphs,
                        if config.options.white_theme {
                            Color::BLACK
                        } else {
                            Color::WHITE
                        },
                        Pos {
                            x: 50.0,
                            y: TASKBAR_HEIGHT + 215.0,
                        },
                        "I hope you enjoy the app as it took a long time to make!",
                        20,
                    );

                    draw_text(
                        &c,
                        g,
                        glyphs,
                        if config.options.white_theme {
                            Color::BLACK
                        } else {
                            Color::WHITE
                        },
                        Pos {
                            x: 50.0,
                            y: TASKBAR_HEIGHT + 285.0,
                        },
                        "Bunch-of-cells",
                        30,
                    );

                    draw_text(
                        &c,
                        g,
                        glyphs,
                        if config.options.white_theme {
                            Color::BLACK
                        } else {
                            Color::WHITE
                        },
                        Pos {
                            x: 50.0,
                            y: TASKBAR_HEIGHT + 340.0,
                        },
                        "Hi! This is a 14yo Bunch-of-cells, who is made up of a bunch of cells (all cells ",
                        20,
                    );
                    draw_text(
                        &c,
                        g,
                        glyphs,
                        if config.options.white_theme {
                            Color::BLACK
                        } else {
                            Color::WHITE
                        },
                        Pos {
                            x: 50.0,
                            y: TASKBAR_HEIGHT + 370.0,
                        },
                        "aren't 14yo). I like programming and playing chess, my main programming language ",
                        20,
                    );
                    draw_text(
                        &c,
                        g,
                        glyphs,
                        if config.options.white_theme {
                            Color::BLACK
                        } else {
                            Color::WHITE
                        },
                        Pos {
                            x: 50.0,
                            y: TASKBAR_HEIGHT + 400.0,
                        },
                        "being Rust. Check out My Github for more such awesome projects. Enjoy! (This ",
                        20,
                    );
                    draw_text(
                        &c,
                        g,
                        glyphs,
                        if config.options.white_theme {
                            Color::BLACK
                        } else {
                            Color::WHITE
                        },
                        Pos {
                            x: 50.0,
                            y: TASKBAR_HEIGHT + 430.0,
                        },
                        "project took a lot of hard work)",
                        20,
                    );
                }
                _ => (),
            }

            // Update glyphs before rendering
            glyphs.factory.encoder.flush(device);
        });
    }
}
