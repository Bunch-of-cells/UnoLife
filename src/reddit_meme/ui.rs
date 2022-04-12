use crate::components::application::MiniApp;
// use crate::menu::ui::TOP_PAD;
use crate::menu::{config::Config, highscores::HighScores};
use crate::{components, Event};
// use http_req::response::{Headers, Response, Status, StatusCode};
// use piston_window::rectangle::square;
use piston_window::*;
use std::ffi::OsStr;
use std::io::Write;
use std::time::Duration;

use std::path::Path;

use http_req::{request::Request, uri::Uri};

pub static mut UPDATE: bool = false;

pub struct MemeApp {
    pub ratelimit_reset: Option<Duration>,
    pub ratelimit_remaining: Option<u32>,
}

impl MemeApp {
    pub fn new() -> Self {
        MemeApp {
            ratelimit_reset: None,
            ratelimit_remaining: None,
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
            println!("No meme file");
            let mut writer = Vec::new(); //container for body of a response
            let res =
                Request::new(&Uri::try_from("https://api.reddit.com/r/memes/random.json").unwrap())
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
                let meme_score = json[0]["data"]["children"][0]["data"]["score"]
                    .as_i64()
                    .unwrap();
                let meme_id = json[0]["data"]["children"][0]["data"]["id"]
                    .as_str()
                    .unwrap()
                    .to_string();
                let is_nsfw = json[0]["data"]["children"][0]["data"]["over_18"]
                    .as_bool()
                    .unwrap();

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
                }
                let mut file =
                    std::fs::File::create(Path::new(&format!("meme.{}", file_extension))).unwrap();
                file.write_all(&writer).unwrap();
                file.flush().unwrap();

                unsafe {
                    UPDATE = false;
                }

                let mut texture_context = window.create_texture_context();
                // make texture for image from response
                let texture = Texture::from_path(
                    &mut texture_context,
                    Path::new(&format!("meme.{}", file_extension)),
                    Flip::None,
                    &TextureSettings::new(),
                )
                .unwrap();

                // delete image
                // std::fs::remove_file(Path::new(&format!("meme.{}", Path::new(meme_url).extension().unwrap().to_str().unwrap_or_else(|| "jpg")))).unwrap();

                let width = texture
                    .get_width()
                    .min(components::application::DEFAULT_WIDTH);
                let height = texture
                    .get_height()
                    .min(components::application::DEFAULT_HEIGHT);

                window.set_size([width as u32, height as u32]);

                // set window title to meme title
                window.set_title(meme_title);
                // show window
                window.show();

                let image = Image::new().rect([0.0, 0.0, width as f64, height as f64]);

                window.draw_2d(event, |c, g, _| {
                    clear([1.0; 4], g);
                    // draw image with texture
                    image.draw(&texture, &DrawState::new_alpha(), c.transform, g);
                });
            } else {
                // something went wrong, most likely rate limited
                println!(
                    "{} / {} / {}",
                    res.status_code(),
                    res.reason(),
                    res.headers()
                );
            }
        }
    }
}
