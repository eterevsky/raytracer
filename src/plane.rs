use cgmath::{abs_diff_eq, dot, InnerSpace, Point3, Vector3};
use crate::shape::{Intersection, Shape};

pub struct Plane {
    point: Point3<f32>,
    normal: Vector3<f32>,
}

impl Plane {
    pub fn new(point: Point3<f32>, normal: Vector3<f32>) -> Self {
        Plane { point, normal }
    }
}

impl Shape for Plane {
    fn ray_intersect(&self, origin: Point3<f32>, dir: Vector3<f32>) -> Intersection {
        let dir_proj = dot(dir, self.normal);
        if abs_diff_eq!(dir_proj, 0.) {
            return Intersection::no();
        }
        let point_proj = dot(self.point - origin, self.normal);
        let ratio = point_proj / dir_proj;
        if ratio < 1E-6 {
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

