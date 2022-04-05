pub struct Color();

impl Color {
    pub fn new(red: u8, green: u8, blue: u8, alpha: f32) -> [f32; 4] {
        return [red as f32/255.0, green as f32/255.0, blue as f32/255.0, alpha];
    }
}