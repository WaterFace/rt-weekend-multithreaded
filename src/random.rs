use rand::Rng;

pub fn random() -> f32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(0.0..1.0)
}

pub fn random_range(min: f32, max: f32) -> f32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(min..max)
}
