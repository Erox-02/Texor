pub const led_cnt: usize = 84;

#[derive(Clone, Copy, Default)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    pub const off: Self = Self::new(0, 0, 0);
    pub const red: Self = Self::new(255, 0, 0);
    pub const green: Self = Self::new(0, 255, 0);
    pub const blue: Self = Self::new(0, 0, 255);
    pub const white: Self = Self::new(255, 255, 255);
}

pub struct Rgb {
    pub leds: [Color; led_cnt],
}

impl Rgb {
    pub const fn new() -> Self {
        Self {
            leds: [Color::off; led_cnt],
        }
    }

    pub fn set(&mut self, idx: usize, color: Color) {
        if idx < led_cnt {
            self.leds[idx] = color;
        }
    }

    pub fn fill(&mut self, color: Color) {
        self.leds.fill(color);
    }

    pub fn clear(&mut self) {
        self.fill(Color::off);
    }
}