use crate::components::application::MiniApp;
use crate::components::button::{draw_text, Pos};
use crate::components::color::Color;
use crate::menu::ui::TASKBAR_HEIGHT;
use crate::menu::{config::Config, highscores::HighScores};
use crate::{components, Event, ASSETS};
use piston_window::*;
use rand::Rng;
use std::ffi::OsStr;
use std::io::Write;
use std::time::Duration;

use std::path::Path;

use http_req::{request::Request, uri::Uri};

pub static mut UPDATE: bool = true;

pub struct MemeApp {
    pub ratelimit_reset: Option<Duration>,
    pub ratelimit_remaining: Option<u32>,
    texture: Option<G2dTexture>,
}

impl MemeApp {
    pub fn new() -> Self {
        MemeApp {
            ratelimit_reset: None,
            ratelimit_remaining: None,
            texture: None,
        }
    }
}

impl MiniApp for MemeApp {
    fn render(
        &mut self,
        window: &mut PistonWindow,
        event: &Event,
        glyphs: &mut Glyphs,
        config: &mut Config,
        highscores: &mut HighScores,
    ) {
        if unsafe { UPDATE } {
            let mut writer = Vec::new(); //container for body of a response

            // choose randomly between 2 subreddits, but give other subreddit a higher chance
            let subreddit = match rand::thread_rng().gen_range(0..3) {
                0 => "memes",
                _ => "ProgrammerHumor",
            };

            let res = Request::new(
                &Uri::try_from(
                    format!("https://api.reddit.com/r/{}/random.json", subreddit).as_str(),
                )
                .unwrap(),
            )
            .header(
                "User-Agent",
                "windows:com.fireplank.unolife:v1.0.0 (by /u/fireplank)",
            )
            .read_timeout(Some(Duration::from_secs(5)))
            .write_timeout(Some(Duration::from_secs(5)))
            .timeout(Some(Duration::from_secs(5)))
            .send(&mut writer);

            // check if request was successful
            if res.is_err() {
                println!("Error: Could not connect to reddit");

                unsafe {
                    UPDATE = false;
                }
                self.texture = None;
                return;
            }

            let res = res.unwrap();

            // check if redirect
            if res.status_code().is_redirect() {
                // get location header from response
                let location = res.headers().get("location").unwrap();
                Request::new(&Uri::try_from(location.as_str()).unwrap())
                    .header(
                        "User-Agent",
                        "windows:com.fireplank.unolife:v1.0.0 (by /u/fireplank)",
                    )
                    .read_timeout(Some(Duration::from_secs(10)))
                    .write_timeout(Some(Duration::from_secs(10)))
                    .timeout(Some(Duration::from_secs(5)))
                    .send(&mut writer)
                    .unwrap();

                // convert body to json
                let json: serde_json::Value = serde_json::from_slice(&writer).unwrap();
                // get data from json
                let meme_url = json[0]["data"]["children"][0]["data"]["url"]
                    .as_str()
                    .unwrap();
                let meme_title = json[0]["data"]["children"][0]["data"]["title"]
                    .as_str()
                    .unwrap()
                    .to_string();
                let meme_author = json[0]["data"]["children"][0]["data"]["author"]
                    .as_str()
                    .unwrap()
                    .to_string();
                let is_nsfw = json[0]["data"]["children"][0]["data"]["over_18"]
                    .as_bool()
                    .unwrap();

                // check if meme is nsfw
                if is_nsfw {
                    println!("Meme is NSFW, Skipping...");
                    self.render(window, event, glyphs, config, highscores);
                    return;
                }

                // download meme
                let mut writer = Vec::new();
                Request::new(&Uri::try_from(meme_url).unwrap())
                    .header(
                        "User-Agent",
                        "windows:com.fireplank.unolife:v1.0.0 (by /u/fireplank)",
                    )
                    .read_timeout(Some(Duration::from_secs(10)))
                    .write_timeout(Some(Duration::from_secs(10)))
                    .timeout(Some(Duration::from_secs(5)))
                    .send(&mut writer)
                    .unwrap();

                // save image
                let file_extension = Path::new(meme_url)
                    .extension()
                    .unwrap_or_else(|| OsStr::new("jpg"))
                    .to_str()
                    .unwrap_or("jpg");
                if file_extension != "jpg" && file_extension != "png" && file_extension != "jpeg" {
                    // file format not supported, so skip
                    self.render(window, event, glyphs, config, highscores);
                    return;
                }
                let mut file =
                    std::fs::File::create(ASSETS.join(format!("meme.{}", file_extension))).unwrap();
                file.write_all(&writer).unwrap();
                file.flush().unwrap();

                let mut texture_context = window.create_texture_context();
                // make texture for image from response
                let texture = Texture::from_path(
                    &mut texture_context,
                    ASSETS.join(format!("meme.{}", file_extension)),
                    Flip::None,
                    &TextureSettings::new(),
                );

                // delete meme file
                std::fs::remove_file(ASSETS.join(format!("meme.{}", file_extension)))
                    .unwrap_or_else(|err| {
                        println!("Error when trying to delete meme: {}", err);
                    });

                // if texture is not loaded, skip
                if texture.is_err() {
                    println!("Error: Could not load meme");
                    self.render(window, event, glyphs, config, highscores);
                    return;
                }

                let texture = texture.unwrap();

                unsafe {
                    UPDATE = false;
                }

                // set window title to meme title
                window.set_title(format!("UnoLife - {} (by /u/{})", meme_title, meme_author));

                let width = texture
                    .get_width();
                let height = texture
                    .get_height();

                let image = Image::new().rect([
                    0.0,
                    TASKBAR_HEIGHT,
                    width as f64,
                    height as f64 + TASKBAR_HEIGHT,
                ]);

                window.draw_2d(event, |c, g, _| {
                    clear(
                        if config.options.white_theme {
                            Color::DARK_THEME_BG
                        } else {
                            Color::WHITE
                        },
                        g,
                    );
                    // draw image with texture
                    image.draw(
                        self.texture.as_ref().unwrap(),
                        &DrawState::new_alpha(),
                        c.transform,
                        g,
                    );
                });
                self.texture = Some(texture);
            } else {
                // something went wrong, most likely rate limited
                println!(
                    "{} / {} / {}",
                    res.status_code(),
                    res.reason(),
                    res.headers()
                );

                unsafe {
                    UPDATE = false;
                }
                self.texture = None;
            }
        } else {
            if self.texture == None {
                // error occured, so show error message
                window.draw_2d(event, |c, g, _| {
                    clear([1.0; 4], g);
                    draw_text(
                        &c,
                        g,
                        glyphs,
                        Color::LOSE_TEXT,
                        Pos { x: 50.0, y: 300.0 },
                        "An error occured while trying to load the meme. Please try again later.",
                        24,
                    );
                });
                return;
            }

            let width = self
                .texture
                .as_ref()
                .unwrap()
                .get_width()
                .min(components::application::DEFAULT_WIDTH);
            let height = self
                .texture
                .as_ref()
                .unwrap()
                .get_height()
                .min(components::application::DEFAULT_HEIGHT);

            let image = Image::new().rect([
                0.0,
                TASKBAR_HEIGHT,
                width as f64,
                height as f64 - TASKBAR_HEIGHT,
            ]);

            window.draw_2d(event, |c, g, _| {
                clear(
                    if config.options.white_theme {
                        Color::DARK_THEME_BG
                    } else {
                        Color::WHITE
                    },
                    g,
                );
                // draw image with texture
                image.draw(
                    self.texture.as_ref().unwrap(),
                    &DrawState::new_alpha(),
                    c.transform,
                    g,
                );
            });
        }
    }
}
