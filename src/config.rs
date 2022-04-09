use std::{env, fs::File, io::Write, path::Path};

extern crate serde;
use serde::{Deserialize, Serialize};

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
        if folder.is_empty() {
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
                config_file
                    .write_all(serde_json::to_string(&config).unwrap().as_bytes())
                    .unwrap();
            }
        }

        config.load_config(file);
        config
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
