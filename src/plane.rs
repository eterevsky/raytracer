use cgmath::{abs_diff_eq, dot, InnerSpace, Point3, Vector3};
use crate::shape::*;

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

pub struct PlaneN {
    point: nalgebra::Point3<f32>,
    normal: nalgebra::Unit<nalgebra::Vector3<f32>>,
}

impl PlaneN {
    pub fn new(point: nalgebra::Point3<f32>, normal: nalgebra::Unit<nalgebra::Vector3<f32>>) -> Self {
        PlaneN { point, normal }
    }
}

impl ShapeN for PlaneN {
    fn ray_intersect(&self, origin: nalgebra::Point3<f32>, dir: nalgebra::Unit<nalgebra::Vector3<f32>>) -> IntersectionN {
        let dir_proj = dir.dot(&self.normal);
        if abs_diff_eq!(dir_proj, 0.) {
            return IntersectionN::new_empty();
        }
        let point_proj = self.normal.dot(&(self.point - origin));
        let ratio = point_proj / dir_proj;
        if ratio < 1E-6 {
            return IntersectionN::new_empty();
        }
        IntersectionN::new(ratio, self.normal)
    }
}

mod tests {
    use super::*;

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

#[test]
fn planen_ray_intersect1() {
    let plane = PlaneN::new(nalgebra::Point3::new(0., 0., 1.), nalgebra::Unit::new_normalize(nalgebra::Vector3::new(0., 0., -1.)));
    let intersection = plane.ray_intersect(nalgebra::Point3::new(0., 0., 0.), nalgebra::Unit::new_normalize(nalgebra::Vector3::new(0., 0., 1.)));
    assert_eq!(intersection.dist, 1.)
}

#[test]
fn planen_ray_intersect3() {
    let plane = PlaneN::new(nalgebra::Point3::new(0., 0., 2.), nalgebra::Unit::new_normalize(nalgebra::Vector3::new(0., 0., -1.)));
    let intersection = plane.ray_intersect(nalgebra::Point3::new(0., 0., 0.), nalgebra::Unit::new_normalize(nalgebra::Vector3::new(0., 0., 1.)));
    assert_eq!(intersection.dist, 2.)
}

#[test]
fn planen_ray_intersect4() {
    let plane = PlaneN::new(nalgebra::Point3::new(0., -1., 0.), nalgebra::Unit::new_normalize(nalgebra::Vector3::new(0., 1., 0.)));
    let intersection = plane.ray_intersect(nalgebra::Point3::new(0., 0., -1.), nalgebra::Unit::new_normalize(nalgebra::Vector3::new(0., -0.5, 2.)));
    assert_eq!(intersection.dist, 17f32.sqrt());
}

}