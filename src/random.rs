use glam::{vec3, Vec3};
use rand::Rng;

pub fn random() -> f32 {
    random_range(0.0, 1.0)
}

pub fn random_range(min: f32, max: f32) -> f32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(min..max)
}

pub fn random_vec() -> Vec3 {
    vec3(random(), random(), random())
}

pub fn random_vec_range(min: f32, max: f32) -> Vec3 {
    vec3(
        random_range(min, max),
        random_range(min, max),
        random_range(min, max),
    )
}

pub fn random_in_unit_sphere() -> Vec3 {
    loop {
        let p = random_vec_range(-1.0, 1.0);
        if p.length_squared() >= 1.0 {
            continue;
        }
        return p;
    }
}

pub fn random_unit_vector() -> Vec3 {
    random_in_unit_sphere().normalize()
}
