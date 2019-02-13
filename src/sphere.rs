use crate::shape::{Intersection, Shape};
use cgmath::{dot, BaseFloat, InnerSpace, Point3, Vector3};

pub struct Sphere<S: BaseFloat> {
    center: Point3<S>,
    radius: S,
}

impl<S: BaseFloat> Sphere<S> {
    pub fn new(center: Point3<S>, radius: S) -> Self {
        Sphere { center, radius }
    }
}

impl<S: BaseFloat> Shape<S> for Sphere<S> {
    fn ray_intersect(&self, origin: Point3<S>, dir: Vector3<S>) -> Intersection<S> {
        // assert_relative_eq!(dir.magnitude2(), S::one());
        let to_center = self.center - origin;
        let p = dot(to_center, dir);
        if p < S::zero() {
            return Intersection::no();
        }
        let projection2 = p * p / dir.magnitude2();
        let ray_dist2 = to_center.magnitude2() - projection2;
        let r2 = self.radius * self.radius;
        if ray_dist2 > r2 {
            return Intersection::no();
        }
        let seg2 = r2 - ray_dist2;
        if projection2 < seg2 {
            return Intersection::no();
        }
        let four = S::one() + S::one() + S::one() + S::one();
        let dist2 = projection2 + seg2 - (four * projection2 * seg2).sqrt();
        if dist2 < S::default_epsilon() {
            return Intersection::no();
        }
        let to_intersect = dir * (dist2 / dir.magnitude2()).sqrt();
        Intersection::new(dist2, to_intersect - to_center)
    }
}

#[test]
fn sphere_ray_intersect1() {
    let sphere = Sphere::new(Point3::new(0., 0., 2.), 1.);
    let intersection = sphere.ray_intersect(Point3::new(0., 0., 0.), Vector3::new(0., 0., 1.));
    assert_eq!(intersection.dist2, 1.)
}

#[test]
fn sphere_ray_intersect2() {
    let sphere = Sphere::new(Point3::new(0., 0., 3.), 1.);
    let intersection = sphere.ray_intersect(Point3::new(0., 0., 0.), Vector3::new(0., 0., 1.));
    assert_eq!(intersection.dist2, 4.)
}

#[test]
fn sphere_ray_intersect3() {
    let sphere = Sphere::new(Point3::new(0., 0., 2.), 1.);
    let intersection = sphere.ray_intersect(Point3::new(0., 0., 0.), Vector3::new(0., 0., 1.));
    assert_eq!(intersection.dist2, 1.)
}

#[test]
fn sphere_ray_intersect4() {
    let sphere = Sphere::new(Point3::new(0., 0., 3.), 1.);
    let intersection = sphere.ray_intersect(Point3::new(0., 0., 0.), Vector3::new(0., 0., 1.));
    assert_eq!(intersection.dist2, 4.)
}

#[test]
fn sphere_ray_intersect5() {
    let sphere = Sphere::new(Point3::new(0., 0., 3.), 1.);
    let intersection1 = sphere.ray_intersect(Point3::new(0., 0., 0.), Vector3::new(0., -1., 3.));
    let intersection2 = sphere.ray_intersect(Point3::new(0., 0., 0.), Vector3::new(0., -0.5, 1.5));
    assert_eq!(intersection1.dist2, intersection2.dist2);
    assert!(intersection1.dist2 < 6.5);
    assert!(intersection1.dist2 > 6.3);
}
