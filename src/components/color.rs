#[macro_export]
macro_rules! rgb {
    ($r:expr, $g:expr, $b:expr) => {
        [$r as f32 / 255.0, $g as f32 / 255.0, $b as f32 / 255.0, 1.0]
    };

    ($r:expr, $g:expr, $b:expr, $a: expr) => {
        [$r as f32 / 255.0, $g as f32 / 255.0, $b as f32 / 255.0, $a]
    };

    (f32 $r:expr, $g:expr, $b:expr) => {
        [$r, $g, $b, 1.0]
    };

    (f32 $r:expr, $g:expr, $b:expr, $a: expr) => {
        [$r, $g, $b, $a]
    };
}

pub struct Color;

impl Color {
    pub const WHITE: [f32; 4] = rgb!(255, 255, 255);
    pub const BLACK: [f32; 4] = rgb!(0, 0, 0);
    pub const CLEAR: [f32; 4] = rgb!(0, 0, 0, 0.0);
    pub const RESET: [f32; 4] = rgb!(242, 87, 87, 0.9);
    pub const LOSE_TEXT: [f32; 4] = rgb!(242, 87, 87);
    pub const WIN_TEXT: [f32; 4] = rgb!(43, 255, 0);
    pub const DARK_THEME_BG: [f32; 4] = rgb!(100, 100, 100);
}
