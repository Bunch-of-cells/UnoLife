/*!
    Store and fetch highscores from a json file.
    It's not very secure, but it's all local anyway so it doesn't really
    matter if it's tampered with.
*/

use std::{env, fs::File, io::Write, path::Path};

extern crate serde;
use serde::{Deserialize, Serialize};

pub struct HighScores {
    pub location: String,
    pub scores: HighScoreOptions,
}

impl HighScores {
    pub fn new() -> Self {
        HighScores {
            location: "unolife_highscores.json".to_string(),
            scores: HighScoreOptions::default(),
        }
    }

    pub fn fetch_scores() -> Self {
        let mut highscores = HighScores::new();

        if let Ok(folder) = env::var("localappdata") {
            // make folder if it doesnt exist
            std::fs::create_dir_all(folder.clone() + "\\UnoLife").unwrap();

            highscores.location = folder + "\\UnoLife\\highscores.json";
        }

        // create file on system if it doesnt exist
        if !Path::new(&highscores.location).exists() {
            let mut highscores_file = File::create(&highscores.location).unwrap();
            highscores_file
                .write_all(
                    serde_json::to_string(&HighScoreOptions::default())
                        .unwrap()
                        .as_bytes(),
                )
                .unwrap();
        }

        highscores.load_scores();
        highscores
    }

    pub fn load_scores(&mut self) {
        let highscores_file = File::open(&self.location).unwrap();
        let highscores_json: HighScoreOptions =
            serde_json::from_reader(highscores_file).unwrap_or_default();
        self.scores = highscores_json;
    }

    pub fn save_scores(&self) {
        let highscores_json = serde_json::to_string(&self.scores).unwrap();
        let mut highscores_file = File::create(&self.location).unwrap();
        highscores_file
            .write_all(highscores_json.as_bytes())
            .unwrap();
    }

    pub fn reset_highscores(&mut self) {
        self.scores = HighScoreOptions::default();
    }
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct HighScoreOptions {
    pub tictactoe_lime: u32,
    pub tictactoe_purple: u32,
    pub wordle: u32,
    pub snake: u32,
    pub twenty48: u32,
    pub puzzle15: u32,
}
