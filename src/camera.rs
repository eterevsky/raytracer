use glam::{Mat4, Quat, Vec3};
use std::f32::consts::FRAC_PI_2;

use crate::scene::Scene;

pub struct Camera {
    eye: Vec3,
    target: Vec3,
    up: Vec3,
    horizontal_fov: f32,
    w: u32,
    h: u32,

    w_half: i32,
    h_half: i32,
    view_rotation: Quat,
    scale: f32,
}

impl Camera {
    pub fn new() -> Self {
        let (w, h) = (1280, 720);
        let horizontal_fov = FRAC_PI_2;
        let eye = Vec3::zero();
        let target = -Vec3::unit_z();
        let up = Vec3::unit_y();

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
            view_rotation: create_view_rotation(eye, target, up),
        }
    }

    pub fn set_eye(mut self, eye: Vec3) -> Self {
        self.eye = eye;
        self.view_rotation = create_view_rotation(self.eye, self.target, self.up);
        self
    }

    pub fn set_target(mut self, target: Vec3) -> Self {
        self.target = target;
        self.view_rotation = create_view_rotation(self.eye, self.target, self.up);
        self
    }

    pub fn set_up(mut self, up: Vec3) -> Self {
        self.up = up;
        self.view_rotation = create_view_rotation(self.eye, self.target, self.up);
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

        for (x, y, pixel) in image.enumerate_pixels_mut() {
            let dir = self.pixel_ray(x, y);
            *pixel = scene.ray_color(self.eye, dir, rng);
        }

        image
    }

    /// Transform a ray from camera coordinates (i.e. a vector from origin to (x, y, -1)) into
    /// scene coordinates.
    pub fn transform_ray(&self, camera_ray: Vec3) -> Vec3 {
        (self.view_rotation * camera_ray).normalize()
    }

    /// Convert a pixel to a ray in scene coordinates.
    pub fn pixel_ray(&self, x: u32, y: u32) -> Vec3 {
        let x = (x as i32 - self.w_half) as f32;
        let y = (self.h_half - y as i32) as f32;
        let camera_ray = Vec3::new(x * self.scale, y * self.scale, -1.);
        self.transform_ray(camera_ray)
    }

    /// Generate a random ray within a given pixel.
    pub fn sample_pixel_ray(&self, x: u32, y: u32, rng: &mut impl rand::Rng) -> Vec3 {
        let x = (x as i32 - self.w_half) as f32 + rng.gen::<f32>();
        let y = (self.h_half - y as i32) as f32 + rng.gen::<f32>();
        let camera_ray = Vec3::new(x * self.scale, y * self.scale, -1.);
        self.transform_ray(camera_ray)
    }
}

/// Calculates the scale factor
fn scale_from_dims(w: u32, horizontal_fov: f32) -> f32 {
    (0.5 * horizontal_fov).tan() / (0.5 * w as f32)
}

/// Create view transformation that will map a view from (0, 0, 0) to (0, 0, -1) with Y direction up
/// into the view from eye to target.
fn create_view_rotation(eye: Vec3, target: Vec3, up: Vec3) -> Quat {
    let dir = (target - eye).normalize();
    let rotation_mat = Mat4::look_at_rh(Vec3::zero(), dir, up).inverse();
    Quat::from_rotation_mat4(&rotation_mat)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn transform_ray_look_up() {
        let camera = Camera::new()
            .set_target(Vec3::unit_y())
            .set_up(Vec3::unit_z());
        assert!((camera.transform_ray(-Vec3::unit_z()) - Vec3::unit_y()).length() < 1E-6);
        assert!(
            (camera.transform_ray(Vec3::new(1., 1., -1.)) - Vec3::new(1., 1., 1.).normalize())
                .length()
                < 1E-6
        );
    }

    #[test]
    fn transform_ray_translate() {
        let camera = Camera::new()
            .set_eye(Vec3::new(1., 2., -3.))
            .set_target(Vec3::zero())
            .set_up(Vec3::unit_y());
        assert!(
            (camera.transform_ray(Vec3::new(0., 0., -1.)) - Vec3::new(-1., -2., 3.).normalize())
                .length()
                < 1E-6
        )
    }

    #[test]
    fn pixel_ray() {
        let camera = Camera::new().set_fov(FRAC_PI_2).set_dimensions(400, 200);
        assert!((camera.pixel_ray(200, 100) - -Vec3::unit_z()).length() < 1E-6);
        assert!((camera.pixel_ray(0, 100) - Vec3::new(-1., 0., -1.).normalize()).length() < 1E-6);

        assert!((camera.pixel_ray(400, 0) - Vec3::new(1., 0.5, -1.).normalize()).length() < 1E-6);
    }

    #[test]
    fn pixel_ray_with_transform() {
        let camera = Camera::new()
            .set_fov(FRAC_PI_2)
            .set_dimensions(400, 200)
            .set_eye(Vec3::new(1., 2., -3.))
            .set_target(Vec3::zero())
            .set_up(Vec3::unit_y());

        assert!((camera.pixel_ray(200, 100) - Vec3::new(-1., -2., 3.).normalize()).length() < 1E-6);
    }
}
