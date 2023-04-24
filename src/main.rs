use itertools::iproduct;
use rayon::prelude::*;
use rt_weekend_multithreaded::{
    color::Color,
    hit::{Hit, HittableList},
    random::random,
    ray::Ray,
    scene::random_scene,
};
use std::{
    sync::{atomic::AtomicU32, mpsc, Arc},
    time::Instant,
};

use chrono::Local;
use image::{ImageBuffer, Rgb, RgbImage};

fn ray_color(r: &Ray, world: &HittableList, depth: i32) -> Color {
    if depth <= 0 {
        return Color::rgb(0.0, 0.0, 0.0);
    }
    let hit = world.hit(r, 0.001, f32::INFINITY);
    if let Some(rec) = hit {
        if let Some((attenuation, scattered)) = rec.material.scatter(&r, &rec) {
            return attenuation * ray_color(&scattered, world, depth - 1);
        }
        return Color::rgb(0.0, 0.0, 0.0);
    }

    let dir = r.direction.normalize_or_zero();
    let t = 0.5 * (dir.y + 1.0);
    (1.0 - t) * Color::rgb(1.0, 1.0, 1.0) + t * Color::rgb(0.5, 0.7, 1.0)
}

fn main() {
    let start = Instant::now();

    let samples_per_pixel = 500;
    let max_depth = 50;

    let scene = random_scene();

    let image_width: u32 = 1200;
    let image_height: u32 = (image_width as f32 / scene.camera.aspect_ratio) as u32;

    let total_pixels = image_width * image_height;
    let report_every = total_pixels / 100;
    let pixels_completed = Arc::new(AtomicU32::new(0));

    let (sender, reciever) = mpsc::channel();
    iproduct!(0..image_width, 0..image_height)
        .par_bridge()
        .for_each_with(
            (sender, pixels_completed.clone()),
            |(sender, counter), (i, j)| {
                let mut pixel_color = Color::rgb(0.0, 0.0, 0.0);
                for _ in 0..samples_per_pixel {
                    let u = (i as f32 + random()) / (image_width as f32 - 1.0);
                    let v = (j as f32 + random()) / (image_height as f32 - 1.0);
                    let v = 1.0 - v;

                    let r = scene.camera.get_ray(u, v);

                    pixel_color += ray_color(&r, &scene.world, max_depth);
                }

                sender
                    .send((
                        (i, j),
                        pixel_color.to_pixel_color_correction(samples_per_pixel),
                    ))
                    .expect("failed to send message");
                let i = counter.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                if i % report_every == 0 {
                    println!("{}% complete", (i * 100) / total_pixels);
                }
            },
        );

    println!("Writing image...");
    let mut img: RgbImage = ImageBuffer::new(image_width, image_height);
    loop {
        match reciever.recv() {
            Ok(((x, y), pixel)) => img.put_pixel(x, y, Rgb::from(pixel)),
            Err(_) => break,
        }
    }

    let end = Instant::now();
    println!("Took {} seconds", end.duration_since(start).as_secs_f32());

    let now = Local::now();
    let filename = format!("{}.png", now.format("%d-%m-%Y %H%M%S"));
    img.save(filename).expect("Failed to save image");
}
