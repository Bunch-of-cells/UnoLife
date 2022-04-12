use piston_window::{Event, Glyphs, PistonWindow};

use crate::menu::{config::Config, highscores::HighScores};

pub trait MiniApp {
    /// Called to draw on the screen.
    fn render(
        &mut self,
        windows: &mut Vec<PistonWindow>,
        event: &Event,
        glyphs: &mut Glyphs,
        config: &mut Config,
        highscores: &mut HighScores,
    );
}

pub const DEFAULT_WIDTH: u32 = 1024;
pub const DEFAULT_HEIGHT: u32 = 576;
