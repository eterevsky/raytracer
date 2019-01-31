use image;
use num_traits::cast::{NumCast, cast};
use num_traits::cast::AsPrimitive;
use num_traits::float::Float;
use num_traits::identities::{one, zero};
use std::cmp::{Ordering, PartialOrd};
use std::ops;
use std::time;

#[derive(Clone, Copy, Debug)]
pub struct Vec3<S: Float> (pub S, pub S, pub S);

impl<S: Float> Vec3<S> {
    pub fn zero() -> Self {
        Vec3(zero(), zero(), zero())
    }

    fn norm(self) -> S {
        self.0 * self.0 + self.1 * self.1 + self.2 * self.2
    }

    pub fn len(self) -> S {
        self.norm().sqrt()
    }

    pub fn normalize(self) -> Self {
        let len = self.len();
        Vec3(self.0 / len, self.1 / len, self.2 / len)
    }
}

impl<S: Float> ops::Add<Vec3<S>> for Vec3<S> {
    type Output = Vec3<S>;

    fn add(self, rhs: Vec3<S>) -> Self::Output {
        Vec3(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl<S: Float> ops::Sub<Vec3<S>> for Vec3<S> {
    type Output = Vec3<S>;

    fn sub(self, rhs: Vec3<S>) -> Self::Output {
        Vec3(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

impl<S: Float> ops::Mul<Vec3<S>> for Vec3<S> {
    type Output = S;

    fn mul(self, rhs: Vec3<S>) -> S {
        self.0 * rhs.0 + self.1 * rhs.1 + self.2 * rhs.2
    }
}

impl<S: Float> ops::Mul<S> for Vec3<S> {
    type Output = Vec3<S>;

    fn mul(self, rhs: S) -> Vec3<S> {
        Vec3(self.0 * rhs, self.1 * rhs, self.2 * rhs)
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

impl<S: Float> ops::Add<Vec3<S>> for Pnt3<S> {
    type Output = Pnt3<S>;

    fn add(self, rhs: Vec3<S>) -> Pnt3<S> {
        Pnt3(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

pub struct Intersection<S: Float> {
    pub dist2: S,
    pub normal: Vec3<S>,
}

impl<S: Float> Intersection<S> {
    pub fn no() -> Self {
        Intersection {
            dist2: -one::<S>(),
            normal: Vec3::zero(),
        }
    }

    pub fn new(dist2: S, normal: Vec3<S>) -> Self {
        Intersection {
            dist2,
            normal,
        }
    }

    pub fn exists(&self) -> bool { self.dist2 >= zero() }
}

impl<S: Float> PartialEq for Intersection<S> {
    fn eq(&self, other: &Intersection<S>) -> bool {
        self.exists() && other.exists() && self.dist2 == other.dist2
    }
}

impl<S: Float> PartialOrd for Intersection<S> {
    fn partial_cmp(&self, other: &Intersection<S>) -> Option<Ordering> {
        if self.exists() {
            if other.exists() {
                self.dist2.partial_cmp(&other.dist2)
            } else {
                Some(Ordering::Less)
            }
        } else {
            if other.exists() {
                Some(Ordering::Greater)
            } else {
                None
            }
        }
    }
}

pub trait Shape<S: Float> {
    // Returns negative value if there is no intersection, or the square distance to
    // the intersection if there is one.
    fn ray_intersect(&self, origin: Pnt3<S>, dir: Vec3<S>) -> Intersection<S>;
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

impl<S: Float + AsPrimitive<f32>> Shape<S> for Sphere<S> {
    fn ray_intersect(&self, origin: Pnt3<S>, dir: Vec3<S>) -> Intersection<S> {
        let to_center = self.center - origin;
        let p = to_center * dir;
        let projection2 = p*p / dir.norm();
        let ray_dist2 = to_center.norm() - projection2;
        let r2 = self.radius * self.radius;
        if ray_dist2 > r2 {
            return Intersection::no();
        }
        let seg2 = r2 - ray_dist2;
        if projection2 < seg2 {
            return Intersection::no();
        }
        let four = one::<S>() + one::<S>() + one::<S>() + one::<S>();
        let dist2 = projection2 + seg2 - (four * projection2 * seg2).sqrt();
        if dist2.as_() < 1E-6 {
            return Intersection::no();
        }
        let to_intersect = dir * (dist2 / dir.norm()).sqrt();
        Intersection::new(dist2, to_intersect - to_center)
    }
}

#[test]
fn sphere_ray_intersect1() {
    let sphere = Sphere::new(Pnt3(0., 0., 2.), 1.);
    let intersection = sphere.ray_intersect(Pnt3(0., 0., 0.), Vec3(0., 0., 1.));
    assert_eq!(intersection.dist2, 1.)
}

#[test]
fn sphere_ray_intersect2() {
    let sphere = Sphere::new(Pnt3(0., 0., 3.), 1.);
    let intersection = sphere.ray_intersect(Pnt3(0., 0., 0.), Vec3(0., 0., 1.));
    assert_eq!(intersection.dist2, 4.)
}

#[test]
fn sphere_ray_intersect3() {
    let sphere = Sphere::new(Pnt3(0., 0., 2.), 1.);
    let intersection = sphere.ray_intersect(Pnt3(0., 0., 0.), Vec3(0., 0., 1.));
    assert_eq!(intersection.dist2, 1.)
}

#[test]
fn sphere_ray_intersect4() {
    let sphere = Sphere::new(Pnt3(0., 0., 3.), 1.);
    let intersection = sphere.ray_intersect(Pnt3(0., 0., 0.), Vec3(0., 0., 1.));
    assert_eq!(intersection.dist2, 4.)
}

#[test]
fn sphere_ray_intersect5() {
    let sphere = Sphere::new(Pnt3(0., 0., 3.), 1.);
    let intersection1 = sphere.ray_intersect(Pnt3(0., 0., 0.), Vec3(0., -1., 3.));
    let intersection2 = sphere.ray_intersect(Pnt3(0., 0., 0.), Vec3(0., -0.5, 1.5));
    assert_eq!(intersection1.dist2, intersection2.dist2);
    assert!(intersection1.dist2 < 6.5);
    assert!(intersection1.dist2 > 6.3);
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

impl<S: Float + AsPrimitive<f32>> Shape<S> for Plane<S> {
    fn ray_intersect(&self, origin: Pnt3<S>, dir: Vec3<S>) -> Intersection<S> {
        let dir_proj = dir * self.normal;
        if dir_proj == zero() {
            return Intersection::no();
        }
        let point_proj = (self.point - origin) * self.normal;
        let ratio = point_proj / dir_proj;
        if ratio < zero() {
            return Intersection::no();
        }
        let dist2 = ratio * ratio * dir.norm();
        if dist2.as_() < 1E-6 {
            return Intersection::no();
        }
        Intersection::new(dist2, self.normal)
    }
}

#[test]
fn plane_ray_intersect1() {
    let plane = Plane::new(Pnt3(0., 0., 1.), Vec3(0., 0., -1.));
    let intersection = plane.ray_intersect(Pnt3(0., 0., 0.), Vec3(0., 0., 1.));
    assert_eq!(intersection.dist2, 1.)
}

#[test]
fn plane_ray_intersect2() {
    let plane = Plane::new(Pnt3(0., 0., 1.), Vec3(0., 0., -2.));
    let intersection = plane.ray_intersect(Pnt3(0., 0., 0.), Vec3(0., 0., 1.));
    assert_eq!(intersection.dist2, 1.)
}

#[test]
fn plane_ray_intersect3() {
    let plane = Plane::new(Pnt3(0., 0., 2.), Vec3(0., 0., -1.));
    let intersection = plane.ray_intersect(Pnt3(0., 0., 0.), Vec3(0., 0., 1.));
    assert_eq!(intersection.dist2, 4.)
}

#[test]
fn plane_ray_intersect4() {
    let plane = Plane::new(Pnt3(0., -1., 0.), Vec3(0., 1., 0.));
    let intersection = plane.ray_intersect(Pnt3(0., 0., -1.), Vec3(0., -0.5, 2.));
    assert_eq!(intersection.dist2, 17.);
}

#[derive(Clone, Copy)]
pub struct Material {
    pub color: image::Rgb<u8>
}

impl Material {
    pub fn new(r: f32, g: f32, b: f32) -> Self {
        Material {
            color: image::Rgb([(r * 255.) as u8, (g * 255.) as u8, (b * 255.) as u8])
        }
    }
}

pub struct PointLight<S: Float> {
    position: Pnt3<S>,
    intensity: S,
}

pub struct Scene<S: Float> {
    spheres: Vec<(usize, Sphere<S>)>,
    planes: Vec<(usize, Plane<S>)>,
    materials: Vec<Material>,
    lights: Vec<PointLight<S>>,
}

impl<S: Float + AsPrimitive<f32>> Scene<S> {
    pub fn new() -> Self {
        Scene {
            spheres: Vec::new(),
            planes: Vec::new(),
            materials: Vec::new(),
            lights: Vec::new()
        }
    }

    pub fn add_sphere(&mut self, sphere: Sphere<S>, material: Material) -> usize {
        let id = self.materials.len();
        self.spheres.push((id, sphere));
        self.materials.push(material);
        id
    }

    pub fn add_plane(&mut self, plane: Plane<S>, material: Material) -> usize {
        let id = self.materials.len();
        self.planes.push((id, plane));
        self.materials.push(material);
        id
    }

    pub fn add_light(&mut self, position: Pnt3<S>, intensity: S) {
        self.lights.push(PointLight{position, intensity})
    }

    pub fn find_intersection(&self, origin: Pnt3<S>, dir: Vec3<S>) -> (Intersection<S>, usize) {
        let mut best_idx = 0;
        let mut nearest = Intersection::no();
        // for idx in 0..self.shapes.len() {
        //     let distance = self.shapes[idx].ray_intersect(origin, dir);
        //     if distance > zero() && distance < nearest {
        //         nearest = distance;
        //         best_idx = Some(idx);
        //     }
        // }

        for (id, sphere) in self.spheres.iter() {
            let intersection = sphere.ray_intersect(origin, dir);
            if intersection < nearest {
                nearest = intersection;
                best_idx = *id;
            }
        }

        for (id, plane) in self.planes.iter() {
            let intersection = plane.ray_intersect(origin, dir);
            if intersection < nearest {
                nearest = intersection;
                best_idx = *id;
            }
        }

        (nearest, best_idx)
    }

    pub fn ray_color(&self, origin: Pnt3<S>, dir: Vec3<S>) -> image::Rgb<u8> {
        let (intersection, id) = self.find_intersection(origin, dir);
        if !intersection.exists() {
            return image::Rgb([0, 0, 0]);
        }

        let material = self.materials[id];
        let ipoint = origin + dir * (intersection.dist2 / dir.norm()).sqrt();
        let mut illumination: S = zero();
        let normal = intersection.normal.normalize();

        for light in self.lights.iter() {
            let light_vec = light.position - ipoint;
            let (to_light_int, _) = self.find_intersection(ipoint, light_vec);
            if to_light_int.exists() { continue; }
            let dist2 = light_vec.norm();
            let prod = normal * light_vec;
            if prod <= zero() { continue; }
            illumination = illumination + light.intensity * prod / dist2;
        }

        image::Rgb([
            (material.color[0] as f32 * illumination.as_() as f32).min(255.) as u8,
            (material.color[1] as f32 * illumination.as_() as f32).min(255.) as u8,
            (material.color[2] as f32 * illumination.as_() as f32).min(255.) as u8,
        ])
    }
}

pub struct Camera<S: Float> {
    w: u32,
    h: u32,
    scale: S,
    origin: Pnt3<S>,
}

impl Camera<f32> {
    // `fov` -- vertical field of view, horizontal field of view scales with 
    pub fn new(w: u32, h: u32, origin: Pnt3<f32>) -> Self {
        Camera {
            w, h, origin,
            scale: 2. / (h as f32)
        }
    }

    pub fn render(&self, scene: &Scene<f32>) -> image::ImageBuffer<image::Rgb<u8>, Vec<u8>> {
        let mut image = image::ImageBuffer::new(self.w, self.h);
        let start = time::Instant::now();
        let mut rays: u64 = 0;

        for (x, y, pixel) in image.enumerate_pixels_mut() {
            rays += 1;
            let x = (x as f32) * self.scale - 1.;
            let y = -(y as f32) * self.scale + 1.;
            let dir = Pnt3(x, y, 0.) - self.origin;
            *pixel = scene.ray_color(self.origin, dir);
        }

        let t = time::Instant::now() - start;
        let t = t.as_secs() as f64 + 1E-9 * t.subsec_nanos() as f64;
        println!("Elapsed {} ms", t * 1000.);
        println!("{} ns per ray", (t / rays as f64) * 1E9);
        image
    }
}