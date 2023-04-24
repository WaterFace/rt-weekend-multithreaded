use glam::Vec3;

use crate::{
    color::Color,
    hit::HitRecord,
    random::{random, random_in_unit_sphere, random_unit_vector},
    ray::Ray,
};

pub enum Material {
    Lambertian { albedo: Color },
    Metal { albedo: Color, fuzz: f32 },
    Dielectric { ior: f32 },
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
            Self::Dielectric { ior } => {
                let attenuation = Color::rgb(1.0, 1.0, 1.0);
                let refraction_ratio = if rec.front_face { 1.0 / ior } else { *ior };

                let unit_direction = r.direction.normalize_or_zero();
                let cos_theta = f32::min(Vec3::dot(-unit_direction, rec.n), 1.0);
                let sin_theta = f32::sqrt(1.0 - cos_theta * cos_theta);

                let cannot_refract = refraction_ratio * sin_theta > 1.0;
                let direction;
                if cannot_refract || reflectance(cos_theta, refraction_ratio) > random() {
                    direction = reflect(unit_direction, rec.n);
                } else {
                    direction = refract(unit_direction, rec.n, refraction_ratio);
                }

                let scattered = Ray {
                    origin: rec.p,
                    direction,
                };

                Some((attenuation, scattered))
            }
        }
    }
}

fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v - 2.0 * Vec3::dot(v, n) * n
}

fn refract(uv: Vec3, n: Vec3, etai_over_etat: f32) -> Vec3 {
    let cos_theta = f32::min(Vec3::dot(-uv, n), 1.0);
    let r_out_perp = etai_over_etat * (uv + cos_theta * n);
    let r_out_parallel = -f32::sqrt(f32::abs(1.0 - r_out_perp.length_squared())) * n;

    r_out_perp + r_out_parallel
}

fn reflectance(cosine: f32, ref_idx: f32) -> f32 {
    let r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    let r0 = r0 * r0;

    r0 + (1.0 - r0) * f32::powi(1.0 - cosine, 5)
}
