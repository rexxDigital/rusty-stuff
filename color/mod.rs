pub struct Color {
    red: u8,
    green: u8,
    blue: u8,
    alpha: u8
}

//Basic rgb color implementation
impl Color {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Color {
        Color {
            red: r,
            green: g,
            blue: b,
            alpha: a
        }
    }
    pub fn newBase(r: f32, g: f32, b: f32, a: f32) -> Color {
        Color {
            red: (r * 255.0) as u8,
            green: (g * 255.0) as u8,
            blue: (b * 255.0) as u8,
            alpha: (a * 255.0) as u8
        }
    }

    pub fn set_color(&mut self, r: u8, g: u8, b: u8, a: u8) {
        self.red = r;
        self.green = g;
        self.blue = b;
        self.alpha = a;
    }

    pub fn get_color(&self) -> (u8, u8, u8, u8) {
        (self.red, self.green, self.blue, self.alpha)
    }

    pub fn r(&self) -> (u8) { (self.red) }
    pub fn g(&self) -> (u8) { (self.green) }
    pub fn b(&self) -> (u8) { (self.blue) }
    pub fn a(&self) -> (u8) { (self.alpha) }

    pub fn rBase(&self) -> (f32) { (self.red as f32 / 255.0) }
    pub fn gBase(&self) -> (f32) { (self.green as f32 / 255.0) }
    pub fn bBase(&self) -> (f32) { (self.blue as f32 / 255.0) }
    pub fn aBase(&self) -> (f32) { (self.alpha as f32 / 255.0) }
}