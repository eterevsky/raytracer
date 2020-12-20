use glam::Vec3;

use crate::defines::*;
use crate::shape::*;

pub struct Plane {
    point: Vec3,
    normal: Vec3,
}

impl Plane {
    pub fn new(point: Vec3, normal: Vec3) -> Self {
        Plane { point, normal: normal.normalize() }
    }
}

impl Shape for Plane {
    fn ray_intersect(&self, origin: Vec3, dir: Vec3) -> Intersection {
        let dir_proj = dir.dot(self.normal);
        if dir_proj > -EPSILON {
            return Intersection::new_empty();
        }
        let point_proj = self.normal.dot(self.point - origin);
        let ratio = point_proj / dir_proj;
        if ratio < EPSILON {
            return Intersection::new_empty();
        }
        Intersection::new(ratio, self.normal)
    }
}

#[cfg(test)]
mod tests {
    use glam::vec3;

    use super::*;

    #[test]
fn plane_ray_intersect1() {
    let plane = Plane::new(vec3(0., 0., 1.), vec3(0., 0., -1.));
    let intersection = plane.ray_intersect(vec3(0., 0., 0.), vec3(0., 0., 1.));
    assert_eq!(intersection.dist, 1.)
}

#[test]
fn plane_ray_intersect2() {
    let plane = Plane::new(vec3(0., 0., 1.), vec3(0., 0., -2.));
    let intersection = plane.ray_intersect(vec3(0., 0., 0.), vec3(0., 0., 1.));
    assert_eq!(intersection.dist, 1.)
}

#[test]
fn plane_ray_intersect3() {
    let plane = Plane::new(vec3(0., 0., 2.), vec3(0., 0., -1.));
    let intersection = plane.ray_intersect(vec3(0., 0., 0.), vec3(0., 0., 1.));
    assert_eq!(intersection.dist, 2.)
}

#[test]
fn plane_ray_intersect4() {
    let plane = Plane::new(vec3(0., -1., 0.), vec3(0., 1., 0.));
    let intersection = plane.ray_intersect(vec3(0., 0., -1.), vec3(0., -0.5, 2.).normalize());
    assert_relative_eq!(intersection.dist, 17f32.sqrt());
}

}