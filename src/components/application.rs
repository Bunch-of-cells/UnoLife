use piston_window::{Event, PistonWindow, Glyphs, Key};

pub trait MiniApp {
    const NAME: &'static str;

    /// Called to draw on the screen.
    fn render(&mut self, window: &mut PistonWindow, event: &Event, glyphs: &mut Glyphs);
}

pub const DEFAULT_WIDTH: u32 = 1280;
pub const DEFAULT_HEIGHT: u32 = 720;
