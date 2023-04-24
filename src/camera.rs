use glam::Vec3;

use crate::{random::random_in_unit_disk, ray::Ray};

pub struct Camera {
    pub aspect_ratio: f32,
    pub focal_length: f32,
    pub origin: Vec3,
    pub lower_left_corner: Vec3,
    pub vertical: Vec3,
    pub horizontal: Vec3,
    pub u: Vec3,
    pub v: Vec3,
    pub w: Vec3,
    pub lens_radius: f32,
}

impl Camera {
    pub fn new(
        pos: Vec3,
        looking_at: Vec3,
        up: Vec3,
        vfov: f32,
        aspect_ratio: f32,
        aperture: f32,
        focus_dist: f32,
    ) -> Self {
        let theta = vfov.to_radians();
        let h = f32::tan(theta / 2.0);
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (pos - looking_at).normalize_or_zero();
        let u = Vec3::cross(up, w).normalize_or_zero();
        let v = Vec3::cross(w, u);

        let origin = pos;
        let horizontal = focus_dist * viewport_width * u;
        let vertical = focus_dist * viewport_height * v;
        let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - focus_dist * w;

        let lens_radius = aperture / 2.0;

        Self {
            aspect_ratio,
            focal_length: 1.0,
            origin,
            lower_left_corner,
            vertical,
            horizontal,
            u,
            v,
            w,
            lens_radius,
        }
    }

    pub fn get_ray(&self, s: f32, t: f32) -> Ray {
        let rd = self.lens_radius * random_in_unit_disk();
        let offset = self.u * rd.x + self.v * rd.y;
        Ray {
            origin: self.origin + offset,
            direction: self.lower_left_corner + s * self.horizontal + t * self.vertical
                - self.origin
                - offset,
        }
    }
}
