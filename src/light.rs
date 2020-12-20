use glam::{Vec3, vec3};
use rand_distr::{UnitSphere, Distribution};

pub trait Light {
    // Returns a vector from `from` to the point of intersection with the light source.
    fn sample_ray<R: rand::Rng>(&self, from: Vec3, rng: &mut R) -> Vec3;
    fn intensity(&self) -> f32;
}

pub struct PointLight {
    position: Vec3,
    intensity: f32,
}

impl PointLight {
    pub fn new(position: Vec3, intensity: f32) -> Self {
        PointLight { position, intensity }
    }
}

impl Light for PointLight {
    fn sample_ray<R: rand::Rng>(&self, from: Vec3, _rng: &mut R) -> Vec3 {
        self.position - from
    }

    fn intensity(&self) -> f32 {
        self.intensity
    }
}

pub struct SphereLight {
    center: Vec3,
    radius: f32,
    intensity: f32,
}

impl SphereLight {
    pub fn new(center: Vec3, radius: f32, intensity: f32) -> Self {
        SphereLight {
            center,
            radius,
            intensity,
        }
    }
}

impl Light for SphereLight {
    fn sample_ray<R: rand::Rng>(&self, from: Vec3, rng: &mut R) -> Vec3 {
        let radial: [f32; 3] = UnitSphere.sample(rng);
        let radial = vec3(radial[0], radial[1], radial[2]);
        // assert!((unit.magnitude2() - S::one()).abs() < S::from(0.00001).unwrap());
        let sphere_point = self.center + radial * self.radius;
        sphere_point - from
    }

    fn intensity(&self) -> f32 {
        self.intensity
    }
}
