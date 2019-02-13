use cgmath::{abs_diff_eq, dot, BaseFloat, InnerSpace, Point3, Vector3};
use crate::shape::{Intersection, Shape};

pub struct Plane<S: BaseFloat> {
    point: Point3<S>,
    normal: Vector3<S>,
}

impl<S: BaseFloat> Plane<S> {
    pub fn new(point: Point3<S>, normal: Vector3<S>) -> Self {
        Plane { point, normal }
    }
}

impl<S: BaseFloat> Shape<S> for Plane<S> {
    fn ray_intersect(&self, origin: Point3<S>, dir: Vector3<S>) -> Intersection<S> {
        let dir_proj = dot(dir, self.normal);
        if abs_diff_eq!(dir_proj, S::zero()) {
            return Intersection::no();
        }
        let point_proj = dot(self.point - origin, self.normal);
        let ratio = point_proj / dir_proj;
        if ratio < S::default_epsilon() {
            return Intersection::no();
        }
        let dist2 = ratio * ratio * dir.magnitude2();
        Intersection::new(dist2, self.normal)
    }
}

#[test]
fn plane_ray_intersect1() {
    let plane = Plane::new(Point3::new(0., 0., 1.), Vector3::new(0., 0., -1.));
    let intersection = plane.ray_intersect(Point3::new(0., 0., 0.), Vector3::new(0., 0., 1.));
    assert_eq!(intersection.dist2, 1.)
}

#[test]
fn plane_ray_intersect2() {
    let plane = Plane::new(Point3::new(0., 0., 1.), Vector3::new(0., 0., -2.));
    let intersection = plane.ray_intersect(Point3::new(0., 0., 0.), Vector3::new(0., 0., 1.));
    assert_eq!(intersection.dist2, 1.)
}

#[test]
fn plane_ray_intersect3() {
    let plane = Plane::new(Point3::new(0., 0., 2.), Vector3::new(0., 0., -1.));
    let intersection = plane.ray_intersect(Point3::new(0., 0., 0.), Vector3::new(0., 0., 1.));
    assert_eq!(intersection.dist2, 4.)
}

#[test]
fn plane_ray_intersect4() {
    let plane = Plane::new(Point3::new(0., -1., 0.), Vector3::new(0., 1., 0.));
    let intersection = plane.ray_intersect(Point3::new(0., 0., -1.), Vector3::new(0., -0.5, 2.));
    assert_eq!(intersection.dist2, 17.);
}

