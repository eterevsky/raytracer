use cgmath::{dot, InnerSpace, Matrix3, Point3, Vector3};
use std::time;

use crate::scene::Scene;

pub struct Camera {
    origin: Point3<f32>,
    direction: Vector3<f32>,
    up: Vector3<f32>,
    horizontal_fov: f32,
    w: u32,
    h: u32,
}

impl Camera {
    pub fn new() -> Self {
        Camera {
            origin: Point3::new(0., 0., 1.),
            direction: Vector3::new(0., 0., -1.),
            up: Vector3::new(0., 1., 0.),
            horizontal_fov: std::f32::consts::FRAC_PI_2,
            w: 1280,
            h: 720,
        }
    }

    pub fn set_origin(mut self, origin: Point3<f32>) -> Self {
        self.origin = origin;
        self
    }

    pub fn set_direction(mut self, direction: Vector3<f32>) -> Self {
        self.direction = direction.normalize();
        self
    }

    pub fn set_up(mut self, up: Vector3<f32>) -> Self {
        self.up = up.normalize();
        self
    }

    pub fn set_fov(mut self, fov: f32) -> Self {
        self.horizontal_fov = fov;
        self
    }

    pub fn set_dimensions(mut self, w: u32, h: u32) -> Self {
        self.w = w;
        self.h = h;
        self
    }

    pub fn render(&self, scene: &Scene) -> image::ImageBuffer<image::Rgb<u8>, Vec<u8>> {
        let project_up = dot(self.direction, self.up);
        let up = (self.up - project_up * self.direction).normalize();
        let left = -self.direction.cross(up);

        let hfov_tan = (self.horizontal_fov / 2.).tan();
        let vfov_tan = (self.h as f32) / (self.w as f32) * hfov_tan;

        let frame_left = self.direction + left * hfov_tan;
        let frame_upleft = frame_left + up * vfov_tan;

        let ray_mat = Matrix3::from_cols(
            -left * hfov_tan * 2. / (self.w as f32),
            -up * vfov_tan * 2. / (self.h as f32),
            frame_upleft
        );

        let mut image = image::ImageBuffer::new(self.w, self.h);
        // let start = time::Instant::now();
        // let mut rays: u64 = 0;

        for (x, y, pixel) in image.enumerate_pixels_mut() {
            // rays += 1;
            let dir = ray_mat * Vector3::new(x as f32, y as f32, 1.);
            let dir = dir.normalize();
            *pixel = scene.ray_color(self.origin, dir);
        }

        // let t = time::Instant::now() - start;
        // println!("Elapsed {} ms", t.as_millis());
        // println!("{} ns per ray", t.as_nanos() / (rays as u128));
        image
    }
}
