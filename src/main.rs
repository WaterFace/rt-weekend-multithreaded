use glam::Vec3;
use itertools::iproduct;
use rayon::prelude::*;
use rt_weekend_multithreaded::{camera::Camera, color::Color, ray::Ray};
use std::sync::mpsc;

use chrono::Local;
use image::{ImageBuffer, Rgb, RgbImage};

fn ray_color(r: &Ray) -> Color {
    let dir = r.direction.normalize_or_zero();
    let t = 0.5 * (dir.y + 1.0);

    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}

fn main() {
    let camera = Camera {
        viewport_height: 2.0,
        aspect_ratio: 16.0 / 9.0,
        focal_length: 1.0,
        origin: Vec3::ZERO,
    };

    let image_width: u32 = 768;
    let image_height: u32 = (image_width as f32 / camera.aspect_ratio) as u32;

    let (sender, reciever) = mpsc::channel();
    iproduct!(0..image_width, 0..image_height)
        .par_bridge()
        .for_each_with(sender, |sender, (i, j)| {
            let u = i as f32 / (image_width as f32 - 1.0);
            let v = 1.0 - (j as f32 / (image_height as f32 - 1.0));

            let r = Ray {
                origin: camera.origin,
                direction: camera.lower_left_corner()
                    + u * camera.horizontal()
                    + v * camera.vertical()
                    - camera.origin,
            };

            let color = ray_color(&r);

            sender
                .send(((i, j), color.to_pixel()))
                .expect("failed to send message");
        });

    let total_pixels = image_width * image_height;
    let report_every = total_pixels / 100;
    let mut pixels_completed = 0;
    let mut img: RgbImage = ImageBuffer::new(image_width, image_height);
    loop {
        match reciever.recv() {
            Ok(((x, y), pixel)) => {
                pixels_completed += 1;
                if pixels_completed % report_every == 0 {
                    println!("{}% complete", (pixels_completed * 100) / total_pixels);
                }
                img.put_pixel(x, y, Rgb::from(pixel))
            }
            Err(_) => break,
        }
    }

    let now = Local::now();
    let filename = format!("{}.png", now.format("%d-%m-%Y %H%M%S"));
    println!("{filename}");
    img.save(filename).expect("Failed to save image");
}
