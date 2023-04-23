use std::ops::{Add, AddAssign, Div, Mul};

use glam::Vec3;

pub struct Color {
    r: f32,
    g: f32,
    b: f32,
}

impl Color {
    pub fn rgb(r: f32, g: f32, b: f32) -> Self {
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

impl AddAssign for Color {
    fn add_assign(&mut self, rhs: Self) {
        self.r.add_assign(rhs.r);
        self.g.add_assign(rhs.g);
        self.b.add_assign(rhs.b);
    }
}

impl Add<Color> for Vec3 {
    type Output = Color;
    fn add(self, rhs: Color) -> Self::Output {
        Color {
            r: self.x.add(rhs.r),
            g: self.y.add(rhs.g),
            b: self.z.add(rhs.b),
        }
    }
}

impl Div<f32> for Color {
    type Output = Color;
    fn div(self, rhs: f32) -> Self::Output {
        Color {
            r: self.r.div(rhs),
            g: self.g.div(rhs),
            b: self.b.div(rhs),
        }
    }
}
