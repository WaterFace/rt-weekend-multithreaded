use glam::Vec3;

use crate::{
    color::Color,
    hit::HitRecord,
    random::{random_in_unit_sphere, random_unit_vector},
    ray::Ray,
};

pub enum Material {
    Lambertian { albedo: Color },
    Metal { albedo: Color, fuzz: f32 },
}

impl Material {
    pub fn scatter(&self, r: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        match self {
            Self::Lambertian { albedo } => {
                let mut scatter_direction = rec.n + random_unit_vector();
                if scatter_direction.length_squared() < 1e-12 {
                    scatter_direction = rec.n;
                }
                let scattered = Ray {
                    origin: rec.p,
                    direction: scatter_direction,
                };
                let attenuation = *albedo;

                Some((attenuation, scattered))
            }
            Self::Metal { albedo, fuzz } => {
                let reflected = reflect(r.direction.normalize_or_zero(), rec.n);
                let scattered = Ray {
                    origin: rec.p,
                    direction: reflected + *fuzz * random_in_unit_sphere(),
                };
                let attenuation = *albedo;
                if Vec3::dot(scattered.direction, rec.n) > 0.0 {
                    Some((attenuation, scattered))
                } else {
                    None
                }
            }
        }
    }
}

fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v - 2.0 * Vec3::dot(v, n) * n
}
