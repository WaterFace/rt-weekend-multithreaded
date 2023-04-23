use glam::Vec3;

use crate::ray::Ray;

pub struct Camera {
    pub aspect_ratio: f32,
    pub viewport_height: f32,
    pub focal_length: f32,
    pub origin: Vec3,
}

impl Camera {
    pub fn viewport_width(&self) -> f32 {
        self.aspect_ratio * self.viewport_height
    }

    pub fn horizontal(&self) -> Vec3 {
        Vec3::new(self.viewport_width(), 0.0, 0.0)
    }

    pub fn vertical(&self) -> Vec3 {
        Vec3::new(0.0, self.viewport_height, 0.0)
    }

    pub fn lower_left_corner(&self) -> Vec3 {
        self.origin
            - self.horizontal() / 2.0
            - self.vertical() / 2.0
            - Vec3::new(0.0, 0.0, self.focal_length)
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        Ray {
            origin: self.origin,
            direction: self.lower_left_corner() + u * self.horizontal() + v * self.vertical()
                - self.origin,
        }
    }
}
