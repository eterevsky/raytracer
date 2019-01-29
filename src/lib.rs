use image::Rgb;
use num_traits::Num;
use num_traits::float::Float;
use num_traits::identities::{one, zero};
use std::ops;

#[derive(Clone, Copy, Debug)]
pub struct Vec3<S: Num + Copy> (pub S, pub S, pub S);

impl<S: Num + Copy> Vec3<S> {
    fn norm(self) -> S {
        self.0 * self.0 + self.1 * self.1 + self.2 * self.2
    }
}

impl Vec3<f32> {
    pub fn len(self) -> f32 {
        self.norm().sqrt()
    }

    pub fn normalize(self) -> Vec3<f32> {
        let len = self.len();
        Vec3(self.0 / len, self.1 / len, self.2 / len)
    }
}

impl<S: Num + Copy> ops::Add<Vec3<S>> for Vec3<S> {
    type Output = Vec3<S>;

    fn add(self, rhs: Vec3<S>) -> Self::Output {
        Vec3(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl<S: Num + Copy> ops::Mul<Vec3<S>> for Vec3<S> {
    type Output = S;

    fn mul(self, rhs: Vec3<S>) -> S {
        self.0 * rhs.0 + self.1 * rhs.1 + self.2 * rhs.2
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Pnt3<S: Float> (pub S, pub S, pub S);

impl<S: Float> ops::Sub<Pnt3<S>> for Pnt3<S> {
    type Output = Vec3<S>;

    fn sub(self, rhs: Pnt3<S>) -> Vec3<S> {
        Vec3(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

pub trait Shape<S: Float> {
    // Returns negative value if there is no intersection, or the square distance to
    // the intersection if there is one.
    fn ray_intersect(&self, origin: Pnt3<S>, dir: Vec3<S>) -> S;
}

pub struct Sphere<S: Float> {
    center: Pnt3<S>,
    radius: S,
}

impl<S: Float> Sphere<S> {
    pub fn new(center: Pnt3<S>, radius: S) -> Self {
        Sphere {
            center,
            radius,
        }
    }
}

impl<S: Float> Shape<S> for Sphere<S> {
    fn ray_intersect(&self, origin: Pnt3<S>, dir: Vec3<S>) -> S {
        let to_center = self.center - origin;
        let p = to_center * dir;
        let projection2 = p*p / dir.norm();
        let ray_dist2 = to_center.norm() - projection2;
        let r2 = self.radius * self.radius;
        if ray_dist2 > r2 {
            return -one::<S>();
        }
        let seg2 = r2 - ray_dist2;
        if projection2 < seg2 {
            return -one::<S>();
        }
        let four = one::<S>() + one::<S>() + one::<S>() + one::<S>();
        projection2 + seg2 - (four * projection2 * seg2).sqrt()
    }
}

#[test]
fn sphere_ray_intersect1() {
    let sphere = Sphere::new(Pnt3(0., 0., 2.), 1.);
    let intersection = sphere.ray_intersect(Pnt3(0., 0., 0.), Vec3(0., 0., 1.));
    assert_eq!(intersection, 1.)
}

#[test]
fn sphere_ray_intersect2() {
    let sphere = Sphere::new(Pnt3(0., 0., 3.), 1.);
    let intersection = sphere.ray_intersect(Pnt3(0., 0., 0.), Vec3(0., 0., 1.));
    assert_eq!(intersection, 4.)
}

#[test]
fn sphere_ray_intersect3() {
    let sphere = Sphere::new(Pnt3(0., 0., 2.), 1.);
    let intersection = sphere.ray_intersect(Pnt3(0., 0., 0.), Vec3(0., 0., 1.));
    assert_eq!(intersection, 1.)
}

#[test]
fn sphere_ray_intersect4() {
    let sphere = Sphere::new(Pnt3(0., 0., 3.), 1.);
    let intersection = sphere.ray_intersect(Pnt3(0., 0., 0.), Vec3(0., 0., 1.));
    assert_eq!(intersection, 4.)
}

#[test]
fn sphere_ray_intersect5() {
    let sphere = Sphere::new(Pnt3(0., 0., 3.), 1.);
    let intersection1 = sphere.ray_intersect(Pnt3(0., 0., 0.), Vec3(0., -1., 3.));
    let intersection2 = sphere.ray_intersect(Pnt3(0., 0., 0.), Vec3(0., -0.5, 1.5));
    assert_eq!(intersection1, intersection2);
    assert!(intersection1 < 6.5);
    assert!(intersection1 > 6.3);
}

pub struct Plane<S: Float> {
    point: Pnt3<S>,
    normal: Vec3<S>,
}

impl<S: Float> Plane<S> {
    pub fn new(point: Pnt3<S>, normal: Vec3<S>) -> Self {
        Plane { point, normal }
    }
}

impl<S: Float + std::fmt::Debug> Shape<S> for Plane<S> {
    fn ray_intersect(&self, origin: Pnt3<S>, dir: Vec3<S>) -> S {
        let dir_proj = dir * self.normal;
        if dir_proj == zero() {
            return -one::<S>();
        }
        let point_proj = (self.point - origin) * self.normal;
        let ratio = point_proj / dir_proj;
        if ratio < zero() {
            return -one::<S>();
        }
        ratio * ratio * dir.norm()
    }
}

#[test]
fn plane_ray_intersect1() {
    let plane = Plane::new(Pnt3(0., 0., 1.), Vec3(0., 0., -1.));
    let intersection = plane.ray_intersect(Pnt3(0., 0., 0.), Vec3(0., 0., 1.));
    assert_eq!(intersection, 1.)
}

#[test]
fn plane_ray_intersect2() {
    let plane = Plane::new(Pnt3(0., 0., 1.), Vec3(0., 0., -2.));
    let intersection = plane.ray_intersect(Pnt3(0., 0., 0.), Vec3(0., 0., 1.));
    assert_eq!(intersection, 1.)
}

#[test]
fn plane_ray_intersect3() {
    let plane = Plane::new(Pnt3(0., 0., 2.), Vec3(0., 0., -1.));
    let intersection = plane.ray_intersect(Pnt3(0., 0., 0.), Vec3(0., 0., 1.));
    assert_eq!(intersection, 4.)
}

#[test]
fn plane_ray_intersect4() {
    let plane = Plane::new(Pnt3(0., -1., 0.), Vec3(0., 1., 0.));
    let intersection = plane.ray_intersect(Pnt3(0., 0., -1.), Vec3(0., -0.5, 2.));
    assert_eq!(intersection, 17.);
}

#[derive(Clone, Copy)]
pub struct Material {
    pub color: Rgb<u8>
}

impl Material {
    pub fn new(r: f32, g: f32, b: f32) -> Self {
        Material {
            color: Rgb([(r * 255.) as u8, (g * 255.) as u8, (b * 255.) as u8])
        }
    }
}

pub struct Scene<S: Float> {
    shapes: Vec<Box<Shape<S>>>,
    materials: Vec<Material>
}

impl<S: Float> Scene<S> {
    pub fn new() -> Self {
        Scene {
            shapes: Vec::new(),
            materials: Vec::new(),
        }
    }

    pub fn add<T: Shape<S> + 'static>(&mut self, shape: T, material: Material) -> usize {
        let idx = self.shapes.len();
        self.shapes.push(Box::new(shape));
        self.materials.push(material);
        idx
    }

    pub fn find_intersection(&self, origin: Pnt3<S>, dir: Vec3<S>) -> Option<Material> {
        let mut best_idx = None;
        let mut nearest = S::infinity();
        for (idx, shape) in self.shapes.iter().enumerate() {
            let distance = shape.ray_intersect(origin, dir);
            if distance > zero() && distance < nearest {
                nearest = distance;
                best_idx = Some(idx);
            }
        }

        best_idx.map(|idx| self.materials[idx])
    }
}