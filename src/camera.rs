use cgmath::{InnerSpace, Point3};
use std::time;

use crate::scene::Scene;

pub struct Camera {
    w: u32,
    h: u32,
    scale: f32,
    origin: Point3<f32>,
}

impl Camera {
    pub fn new(w: u32, h: u32, origin: Point3<f32>) -> Self {
        Camera {
            w,
            h,
            origin,
            scale: 2. / (h as f32)
        }
    }

    pub fn render(&self, scene: &Scene) -> image::ImageBuffer<image::Rgb<u8>, Vec<u8>> {
        let mut image = image::ImageBuffer::new(self.w, self.h);
        let start = time::Instant::now();
        let mut rays: u64 = 0;

        for (x, y, pixel) in image.enumerate_pixels_mut() {
            rays += 1;
            let x = (x as f32) * self.scale - 1.;
            let y = -(y as f32) * self.scale + 1.;
            let dir = Point3::new(x, y, 0.) - self.origin;
            let dir = dir.normalize();
            *pixel = scene.ray_color(self.origin, dir);
        }

        let t = time::Instant::now() - start;
        println!("Elapsed {} ms", t.as_millis());
        println!("{} ns per ray", t.as_nanos() / (rays as u128));
        image
    }
}
