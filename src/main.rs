use itertools::iproduct;
use rayon::prelude::*;
use std::sync::mpsc;

use chrono::Local;
use image::{ImageBuffer, Rgb, RgbImage};

fn main() {
    const WIDTH: u32 = 512;
    const HEIGHT: u32 = 512;

    let mut img: RgbImage = ImageBuffer::new(WIDTH, HEIGHT);

    let (sender, reciever) = mpsc::channel();

    iproduct!(0..WIDTH, 0..HEIGHT)
        .par_bridge()
        .for_each_with(sender, |sender, (i, j)| {
            let r = i as f32 / (WIDTH - 1) as f32;
            let g = j as f32 / (HEIGHT - 1) as f32;
            let b = 0.25;

            let r = (255.999 * r) as u8;
            let g = (255.999 * g) as u8;
            let b = (255.999 * b) as u8;

            sender
                .send(((i, j), [r, g, b]))
                .expect("failed to send message");
        });

    let total_pixels = WIDTH * HEIGHT;
    let report_every = total_pixels / 100;
    let mut pixels_completed = 0;

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
