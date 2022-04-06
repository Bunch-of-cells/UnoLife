use piston_window::{Event, PistonWindow, RenderArgs, UpdateArgs};

pub trait MiniApp {
    const NAME: &'static str;

    /// Called to draw on the screen.
    fn render(&mut self, window: &mut PistonWindow, event: &Event);
}

pub const DEFAULT_WIDTH: u32 = 1280;
pub const DEFAULT_HEIGHT: u32 = 720;
