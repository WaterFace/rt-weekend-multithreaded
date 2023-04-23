use std::ops::{Add, Mul};

pub struct Color {
    r: f32,
    g: f32,
    b: f32,
}

impl Color {
    pub fn new(r: f32, g: f32, b: f32) -> Self {
        Self { r, g, b }
    }

    pub fn to_pixel(&self) -> [u8; 3] {
        let r = (255.999 * self.r) as u8;
        let g = (255.999 * self.g) as u8;
        let b = (255.999 * self.b) as u8;

        [r, g, b]
    }
}

impl Mul<Color> for f32 {
    type Output = Color;
    fn mul(self, rhs: Color) -> Color {
        Color {
            r: self.mul(rhs.r),
            g: self.mul(rhs.g),
            b: self.mul(rhs.b),
        }
    }
}

impl Add for Color {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self {
            r: self.r.add(rhs.r),
            g: self.g.add(rhs.g),
            b: self.b.add(rhs.b),
        }
    }
}
