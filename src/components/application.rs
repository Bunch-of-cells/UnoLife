use piston_window::{Event, Glyphs, PistonWindow};
use crate::menu::ui::Config;

pub trait MiniApp {
    /// Called to draw on the screen.
    fn render(&mut self, window: &mut PistonWindow, event: &Event, glyphs: &mut Glyphs, config: &mut Config);
}

pub const DEFAULT_WIDTH: u32 = 1024;
pub const DEFAULT_HEIGHT: u32 = 576;
