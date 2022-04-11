pub mod animations;
pub mod application;
pub mod button;

#[macro_export]
macro_rules! rgb {
    ($r:expr, $g:expr, $b:expr) => {
        [$r as f32 / 255.0, $g as f32 / 255.0, $b as f32 / 255.0, 1.0]
    };

    ($r:expr, $g:expr, $b:expr, $a: expr) => {
        [$r as f32 / 255.0, $g as f32 / 255.0, $b as f32 / 255.0, $a]
    };
}
