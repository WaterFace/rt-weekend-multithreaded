use glam::Vec3;

use crate::{material::Material, ray::Ray, sphere::Sphere};

pub struct HitRecord<'a> {
    pub p: Vec3,
    pub n: Vec3,
    pub t: f32,
    pub material: &'a Material,
    pub front_face: bool,
}

impl<'a> HitRecord<'a> {
    pub fn with_outward_normal(
        p: Vec3,
        t: f32,
        material: &'a Material,
        outward_normal: Vec3,
        r: &Ray,
    ) -> Self {
        let front_face = Vec3::dot(r.direction, outward_normal) < 0.0;
        let n = if front_face {
            outward_normal
        } else {
            -outward_normal
        };
        HitRecord {
            p,
            n,
            t,
            material,
            front_face,
        }
    }
}

pub trait Hit {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}

enum ListEntry {
    Sphere(Sphere),
}

pub struct HittableList {
    objects: Vec<ListEntry>,
}

impl HittableList {
    pub fn new() -> Self {
        HittableList {
            objects: Vec::new(),
        }
    }

    pub fn add_sphere(&mut self, sphere: Sphere) {
        self.objects.push(ListEntry::Sphere(sphere));
    }
}

impl Hit for HittableList {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut closest_so_far = t_max;
        let mut rec = None;

        for object in &self.objects {
            let temp_rec = match object {
                ListEntry::Sphere(s) => s.hit(&r, t_min, closest_so_far),
            };

            if let Some(HitRecord { t, .. }) = temp_rec {
                closest_so_far = t;
                rec = temp_rec;
            }
        }

        rec
    }
}
