use nalgebra::{Isometry3, Point3, Unit, Vector3};
use std::f32::consts::FRAC_PI_2;

use crate::scene::Scene;

pub struct Camera {
    eye: Point3<f32>,
    target: Point3<f32>,
    up: Vector3<f32>,
    horizontal_fov: f32,
    w: u32,
    h: u32,

    w_half: i32,
    h_half: i32,
    view: Isometry3<f32>,
    scale: f32,
}

impl Camera {
    pub fn new() -> Self {
        let (w, h) = (1280, 720);
        let horizontal_fov = FRAC_PI_2;
        let eye = Point3::origin();
        let target = Point3::new(0., 0., -1.);
        let up = Vector3::y();

        Camera {
            eye,
            target,
            up,
            horizontal_fov,
            w,
            h,
            w_half: (w / 2) as i32,
            h_half: (h / 2) as i32,
            scale: scale_from_dims(w, horizontal_fov),
            view: create_view_transform(&eye, &target, &up),
        }
    }

    pub fn set_eye(mut self, eye: Point3<f32>) -> Self {
        self.eye = eye;
        self.view = create_view_transform(&self.eye, &self.target, &self.up);
        self
    }

    pub fn set_target(mut self, target: Point3<f32>) -> Self {
        self.target = target;
        self.view = create_view_transform(&self.eye, &self.target, &self.up);
        self
    }

    pub fn set_up(mut self, up: Vector3<f32>) -> Self {
        self.up = up.normalize();
        self.view = create_view_transform(&self.eye, &self.target, &self.up);
        self
    }

    pub fn set_fov(mut self, fov: f32) -> Self {
        self.horizontal_fov = fov;
        self.scale = scale_from_dims(self.w, self.horizontal_fov);
        self
    }

    pub fn set_dimensions(mut self, w: u32, h: u32) -> Self {
        self.w = w;
        self.h = h;
        self.w_half = (w / 2) as i32;
        self.h_half = (h / 2) as i32;
        self.scale = scale_from_dims(self.w, self.horizontal_fov);
        self
    }

    pub fn render(
        &self,
        scene: &Scene,
        rng: &mut impl rand::Rng,
    ) -> image::ImageBuffer<image::Rgb<u8>, Vec<u8>> {
        let mut image = image::ImageBuffer::new(self.w, self.h);
        // let start = time::Instant::now();
        // let mut rays: u64 = 0;

        let cg_eye = cgmath::Point3::new(self.eye.x, self.eye.y, self.eye.z);

        for (x, y, pixel) in image.enumerate_pixels_mut() {
            let dir = self.pixel_ray(x, y);
            let cg_dir = cgmath::Vector3::new(dir.x, dir.y, dir.z);
            *pixel = scene.ray_color(rng, cg_eye, cg_dir);
        }

        // let t = time::Instant::now() - start;
        // println!("Elapsed {} ms", t.as_millis());
        // println!("{} ns per ray", t.as_nanos() / (rays as u128));
        image
    }

    /// Transform a ray from camera coordinates (i.e. a vector from origin to (x, y, -1)) into
    /// scene coordinates.
    pub fn transform_ray(&self, camera_ray: &Vector3<f32>) -> Unit<Vector3<f32>> {
        Unit::new_normalize(self.view * camera_ray)
    }

    /// Convert a pixel to a ray in scene coordinates.
    pub fn pixel_ray(&self, x: u32, y: u32) -> Unit<Vector3<f32>> {
        let x = (x as i32 - self.w_half) as f32;
        let y = (self.h_half - y as i32) as f32;
        self.transform_ray(&Vector3::new(x * self.scale, y * self.scale, -1.))
    }

    /// Generate a random ray within a given pixel.
    pub fn sample_pixel_ray(&self, x: u32, y: u32, rng: &mut impl rand::Rng) -> Unit<Vector3<f32>> {
        let x = (x as i32 - self.w_half) as f32 + rng.gen::<f32>();
        let y = (self.h_half - y as i32) as f32 + rng.gen::<f32>();
        self.transform_ray(&Vector3::new(x * self.scale, y * self.scale, -1.))
    }
}

/// Calculates the scale factor
fn scale_from_dims(w: u32, horizontal_fov: f32) -> f32 {
    (0.5 * horizontal_fov).tan() / (0.5 * w as f32)
}

/// Create view transformation that will map a view from (0, 0, 0) to (0, 0, -1) with Y direction up
/// into the view from eye to target.
fn create_view_transform(
    eye: &Point3<f32>,
    target: &Point3<f32>,
    up: &Vector3<f32>,
) -> Isometry3<f32> {
    Isometry3::look_at_rh(eye, target, up).inverse()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn transform_ray_look_up() {
        let camera = Camera::new()
            .set_target(Point3::new(0., 1., 0.))
            .set_up(Vector3::z());
        assert_relative_eq!(
            camera.transform_ray(&Vector3::new(0., 0., -1.)),
            Vector3::y_axis()
        );
        assert_relative_eq!(
            camera.transform_ray(&Vector3::new(1., 1., -1.)),
            Unit::new_normalize(Vector3::new(1., 1., 1.))
        );
    }

    #[test]
    fn transform_ray_translate() {
        let camera = Camera::new()
            .set_eye(Point3::new(1., 2., -3.))
            .set_target(Point3::origin())
            .set_up(Vector3::y());
        assert_relative_eq!(
            camera.transform_ray(&Vector3::new(0., 0., -1.)),
            Unit::new_normalize(Vector3::new(-1., -2., 3.))
        )
    }

    #[test]
    fn pixel_ray() {
        let camera = Camera::new().set_fov(FRAC_PI_2).set_dimensions(400, 200);
        assert_relative_eq!(camera.pixel_ray(200, 100), -Vector3::z_axis(),);
        assert_relative_eq!(
            camera.pixel_ray(0, 100),
            Unit::new_normalize(Vector3::new(-1., 0., -1.)),
        );

        assert_relative_eq!(
            camera.pixel_ray(400, 0),
            Unit::new_normalize(Vector3::new(1., 0.5, -1.)),
        );
    }

    #[test]
    fn pixel_ray_with_transform() {
        let camera = Camera::new()
            .set_fov(FRAC_PI_2)
            .set_dimensions(400, 200)
            .set_eye(Point3::new(1., 2., -3.))
            .set_target(Point3::origin())
            .set_up(Vector3::y());

        assert_relative_eq!(
            camera.pixel_ray(200, 100),
            Unit::new_normalize(Vector3::new(-1., -2., 3.)),
        );
    }
}
