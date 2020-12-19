use cgmath::{Point3, Vector3};
use rand_distr::{UnitSphere, Distribution};

pub trait Light {
    fn sample_ray<R: rand::Rng>(&self, from: Point3<f32>, rng: &mut R) -> Vector3<f32>;
    fn intensity(&self) -> f32;
}

pub struct PointLight {
    position: Point3<f32>,
    intensity: f32,
}

impl PointLight {
    pub fn new(position: Point3<f32>, intensity: f32) -> Self {
        PointLight { position, intensity }
    }
}

impl Light for PointLight {
    fn sample_ray<R: rand::Rng>(&self, from: Point3<f32>, _rng: &mut R) -> Vector3<f32> {
        self.position - from
    }

    fn intensity(&self) -> f32 {
        self.intensity
    }
}

pub struct SphereLight {
    center: Point3<f32>,
    radius: f32,
    intensity: f32,
}

impl SphereLight {
    pub fn new(center: Point3<f32>, radius: f32, intensity: f32) -> Self {
        SphereLight {
            center,
            radius,
            intensity,
        }
    }
}

impl Light for SphereLight {
    fn sample_ray<R: rand::Rng>(&self, from: Point3<f32>, rng: &mut R) -> Vector3<f32> {
        let unit: [f32; 3] = UnitSphere.sample(rng);
        let unit = Vector3::new(unit[0], unit[1], unit[2]);
        // assert!((unit.magnitude2() - S::one()).abs() < S::from(0.00001).unwrap());
        let sphere_point = self.center + unit * self.radius;
        sphere_point - from
    }

    fn intensity(&self) -> f32 {
        self.intensity
    }
}
