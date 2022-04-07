// use crate::tictactoe::ui::*;
use piston_window::{types::Color, *};

pub struct Pos {
    pub x: f64,
    pub y: f64,
}

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

pub fn draw_text(
    ctx: &Context,
    graphics: &mut G2d,
    glyphs: &mut Glyphs,
    color: [f32; 4],
    pos: Pos,
    text: &str,
    font_size: u32,
) {
    text::Text::new_color(color, font_size)
        .draw(
            text,
            glyphs,
            &ctx.draw_state,
            ctx.transform.trans(pos.x as f64, pos.y as f64),
            graphics,
        )
        .unwrap();
}
