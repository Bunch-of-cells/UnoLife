pub struct Color {
    red: f32,
    green: f32,
    blue: f32,
    alpha: f32,
}

impl Color {
    pub fn new(red: u8, green: u8, blue: u8, alpha: f32) -> Self {
        Color {
            red: red as f32 / 255.0,
            green: green as f32 / 255.0,
            blue: blue as f32 / 255.0,
            alpha,
        }
    }

    pub fn as_array(&self) -> [f32; 4] {
        [self.red, self.green, self.blue, self.alpha]
    }
}
