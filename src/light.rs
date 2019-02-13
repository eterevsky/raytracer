use cgmath::{BaseFloat, Point3, Vector3};
use rand::distributions::Distribution;

pub trait Light<S: BaseFloat> {
    fn sample_ray<R: rand::Rng>(&self, from: Point3<S>, rng: &mut R) -> Vector3<S>;
    fn intensity(&self) -> S;
}

pub struct PointLight<S: BaseFloat> {
    position: Point3<S>,
    intensity: S,
}

impl<S: BaseFloat> PointLight<S> {
    pub fn new(position: Point3<S>, intensity: S) -> Self {
        PointLight { position, intensity }
    }
}

impl<S: BaseFloat> Light<S> for PointLight<S> {
    fn sample_ray<R: rand::Rng>(&self, from: Point3<S>, _rng: &mut R) -> Vector3<S> {
        self.position - from
    }

    fn intensity(&self) -> S {
        self.intensity
    }
}

pub struct SphereLight<S: BaseFloat> {
    center: Point3<S>,
    radius: S,
    intensity: S,
    sphere_dist: rand::distributions::UnitSphereSurface,
}

impl<S: BaseFloat> SphereLight<S> {
    pub fn new(center: Point3<S>, radius: S, intensity: S) -> Self {
        SphereLight {
            center,
            radius,
            intensity,
            sphere_dist: rand::distributions::UnitSphereSurface::new(),
        }
    }
}

impl<S: BaseFloat> Light<S> for SphereLight<S> {
    fn sample_ray<R: rand::Rng>(&self, from: Point3<S>, rng: &mut R) -> Vector3<S> {
        let unit = self.sphere_dist.sample(rng);
        let x = S::from(unit[0]).unwrap();
        let y = S::from(unit[1]).unwrap();
        let z = S::from(unit[2]).unwrap();
        let unit = Vector3::new(x, y, z);
        // assert!((unit.magnitude2() - S::one()).abs() < S::from(0.00001).unwrap());
        let sphere_point = self.center + unit * self.radius;
        sphere_point - from
    }

    fn intensity(&self) -> S {
        self.intensity
    }
}

