use std::{env, fs::File, io::Write, path::Path};

extern crate serde;
use serde::{Deserialize, Serialize};

pub struct Config {
    pub location: String,
    pub options: ConfigOptions,
}

impl Config {
    pub fn new() -> Self {
        Config {
            location: "unolife_config.json".to_string(),
            options: ConfigOptions::default(),
        }
    }

    pub fn fetch_config() -> Self {
        let mut config = Config::new();
        let file;
        if let Ok(folder) = env::var("localappdata") {
            // make folder if it doesnt exist
            std::fs::create_dir_all(folder.clone() + "\\UnoLife").unwrap();

            file = folder + "\\UnoLife\\config.json";
            config.location = file.clone();

            // create file on system if it doesnt exist
            if !Path::new(&file).exists() {
                let mut config_file = File::create(file.clone()).unwrap();
                config_file
                    .write_all(serde_json::to_string(&ConfigOptions::default()).unwrap().as_bytes())
                    .unwrap();
            }
        } else {
            file = "unolife_config.json".to_string();
            config.location = file.clone();
        }

        config.load_config(file);
        config
    }

    pub fn load_config(&mut self, file: String) {
        let config_file = File::open(file).unwrap();
        let config_json: ConfigOptions = serde_json::from_reader(config_file).unwrap();
        self.options = config_json;
    }

    pub fn save_config(&self, file: String) {
        let config_json = serde_json::to_string(&self.options).unwrap();
        let mut config_file = File::create(file).unwrap();
        config_file.write_all(config_json.as_bytes()).unwrap();
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ConfigOptions {
    pub white_theme: bool,
    // TODO: add more config options
}

impl Default for ConfigOptions {
    fn default() -> Self {
        ConfigOptions { white_theme: true }
    }
}
