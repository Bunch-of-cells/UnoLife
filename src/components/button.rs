use crate::tictactoe::ui::*;
use piston_window::{types::Color, *};

pub struct UIButton {
    pub text: String,
    pub color: Color,
    pub text_color: Color,
    pub size: u32,
    pub pos: Pos,
    pub width: f64,
    pub height: f64,
}

impl UIButton {
    pub fn new(
        text: &str,
        color: Color,
        text_color: Color,
        size: u32,
        pos: Pos,
        width: f64,
        height: f64,
    ) -> Self {
        UIButton {
            text: text.to_string(),
            color,
            text_color,
            size,
            pos,
            width,
            height,
        }
    }

    pub fn draw(&self, ctx: &Context, graphics: &mut G2d, glyphs: &mut Glyphs) {
        rectangle(
            self.color,
            [self.pos.x, self.pos.y, self.width, self.height],
            ctx.transform,
            graphics,
        );
        draw_text(
            ctx,
            graphics,
            glyphs,
            self.text_color,
            Pos {
                x: self.pos.x + self.width / 8.5 - self.text.len() as f64,
                y: self.pos.y + self.height / 1.5,
            },
            &self.text,
            self.size,
        );
    }

    pub fn is_over(&self, x: f64, y: f64) -> bool {
        x > self.pos.x
            && x < self.pos.x + self.width
            && y > self.pos.y
            && y < self.pos.y + self.height
    }
}
