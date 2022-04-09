use crate::components::{
    application::MiniApp,
    button::{draw_text, Pos, UIButton},
};
use crate::snake::ui::SnakeApp;
use crate::tictactoe::ui::TicTacToeApp;
use crate::wordle::ui::WordleApp;

use piston_window::*;
use std::{env, io::Write, fs::File, path::Path};

extern crate serde;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Config {
    pub location: String,
    pub white_theme: bool,
    // TODO: add more config options
}

impl Config {
    pub fn new() -> Self {
        Config {
            location: "unolife_config.json".to_string(),
            white_theme: true,
        }
    }

    pub fn fetch_config() -> Self {
        let mut config = Config::new();
        let folder = env::var("localappdata").unwrap_or_else(|_| "".to_string());
        let file;
        if folder == "" {
            file = "unolife_config.json".to_string();
            config.location = file.clone();
        } else {
            // make folder if it doesnt exist
            std::fs::create_dir_all(folder.clone() + "\\UnoLife").unwrap();

            file = folder + "\\UnoLife\\config.json";
            config.location = file.clone();

            // create file on system if it doesnt exist
            if !Path::new(&file).exists() {
                let mut config_file = File::create(file.clone()).unwrap();
                config_file.write_all(serde_json::to_string(&config).unwrap().as_bytes())
                    .unwrap();
            }
        }

        config.load_config(file);
        return config;
    }

    pub fn load_config(&mut self, file: String) {
        let config_file = File::open(file).unwrap();
        let config_json: Config = serde_json::from_reader(config_file).unwrap();
        self.white_theme = config_json.white_theme;
    }

    pub fn save_config(&self, file: String) {
        let config_json = serde_json::to_string(&self).unwrap();
        let mut config_file = File::create(file).unwrap();
        config_file.write_all(config_json.as_bytes()).unwrap();
    }
}

pub const TOP_PAD: f64 = 104.0;

const GAMES: usize = 3;
pub struct MainMenu {
    pub hover_pos: [f64; 2],
    pub tab: usize,
    apps: [Box<dyn MiniApp>; GAMES],
    pub config: Config, 
}

impl MainMenu {
    pub fn new() -> Self {
        MainMenu {
            hover_pos: [0.0, 0.0],
            tab: 0,
            config: Config::fetch_config(),
            // make list that contains all apps
            apps: [
                Box::new(TicTacToeApp::new()),
                Box::new(WordleApp::new()),
                Box::new(SnakeApp::new()),
            ],
        }
    }
}

impl MiniApp for MainMenu {
    fn render(&mut self, window: &mut PistonWindow, event: &Event, glyphs: &mut Glyphs, config: &mut Config) {
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
            UIButton::new(
                "Play Snake",
                [0.0; 4],
                [0.0, 0.0, 0.0, 1.0],
                24,
                Pos { x: 40.0, y: 240.0 },
                224.0,
                56.0,
            ),
        ];
        
        let mut config_buttons = [
            UIButton::new(
                "Toggle Theme",
                [0.0; 4],
                [0.0, 0.0, 0.0, 1.0],
                24,
                Pos { x: 40.0, y: 120.0 },
                224.0,
                56.0,
            ),
        ];

        let left_click = event.press_args() == Some(Button::Mouse(MouseButton::Left));

        // handle button events
        for (index, button) in buttons.iter_mut().enumerate() {
            if index < 3 || self.tab == 1 {
                if button.is_over(self.hover_pos[0], self.hover_pos[1]) {
                    if left_click {
                        self.tab = index;
                    } else {
                        button.color = [120.0 / 255.0, 120.0 / 255.0, 120.0 / 255.0, 0.35];
                    }
                }
            }
        }

        // handle config button events
        for (index, button) in config_buttons.iter_mut().enumerate() {
            if button.is_over(self.hover_pos[0], self.hover_pos[1]) {
                if left_click {
                    if index == 0 {
                        config.white_theme = !config.white_theme;
                        config.save_config(config.location.clone());
                    }
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
            _ => self.apps[self.tab - 3].render(window, event, glyphs, config),
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
