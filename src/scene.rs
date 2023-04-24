use glam::{vec3, Vec3};

use crate::{
    camera::Camera,
    color::Color,
    hit::HittableList,
    material::Material,
    random::{random, random_range, random_vec, random_vec_range},
    sphere::Sphere,
};

pub struct Scene {
    pub world: HittableList,
    pub camera: Camera,
}

pub fn random_scene() -> Scene {
    let aspect_ratio = 3.0 / 2.0;
    let pos = vec3(13.0, 2.0, 3.0);
    let looking_at = vec3(0.0, 0.0, 0.0);
    let up = Vec3::Y;
    let dist_to_focus = 10.0;
    let aperture = 0.1;
    let camera = Camera::new(
        pos,
        looking_at,
        up,
        20.0,
        aspect_ratio,
        aperture,
        dist_to_focus,
    );

    let mut world = HittableList::new();

    world.add_sphere(Sphere {
        center: vec3(0.0, -1000.0, 0.0),
        radius: 1000.0,
        material: crate::material::Material::Lambertian {
            albedo: Color::rgb(0.5, 0.5, 0.5),
        },
    });

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random();
            let center = vec3(a as f32 + 0.9 * random(), 0.2, b as f32 + 0.9 * random());

            if (center - vec3(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = (random_vec() * random_vec()).into();
                    world.add_sphere(Sphere {
                        center,
                        radius: 0.2,
                        material: Material::Lambertian { albedo },
                    });
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = random_vec_range(0.5, 1.0).into();
                    let fuzz = random_range(0.0, 0.5);
                    world.add_sphere(Sphere {
                        center,
                        radius: 0.2,
                        material: Material::Metal { albedo, fuzz },
                    })
                } else {
                    // glass
                    world.add_sphere(Sphere {
                        center,
                        radius: 0.2,
                        material: Material::Dielectric { ior: 1.5 },
                    })
                }
            }
        }
    }

    world.add_sphere(Sphere {
        center: vec3(0.0, 1.0, 0.0),
        radius: 1.0,
        material: Material::Dielectric { ior: 1.5 },
    });

    world.add_sphere(Sphere {
        center: vec3(-4.0, 1.0, 0.0),
        radius: 1.0,
        material: Material::Lambertian {
            albedo: Color::rgb(0.4, 0.2, 0.1),
        },
    });

    world.add_sphere(Sphere {
        center: vec3(4.0, 1.0, 0.0),
        radius: 1.0,
        material: Material::Metal {
            albedo: Color::rgb(0.7, 0.6, 0.5),
            fuzz: 0.0,
        },
    });

    Scene { world, camera }
}
