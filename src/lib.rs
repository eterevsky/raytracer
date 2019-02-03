use cgmath;
use cgmath::num_traits::AsPrimitive;
use cgmath::{abs_diff_eq, dot, AbsDiffEq, BaseFloat, InnerSpace, One, Point3, Vector3, Zero};
use image;
use std::cmp::{Ordering, PartialOrd};
use std::time;

pub struct Intersection<S: BaseFloat> {
    pub dist2: S,
    pub normal: Vector3<S>,
}

impl<S: BaseFloat> Intersection<S> {
    pub fn no() -> Self {
        Intersection {
            dist2: -S::one(),
            normal: Vector3::zero(),
        }
    }

    pub fn new(dist2: S, normal: Vector3<S>) -> Self {
        Intersection { dist2, normal }
    }

    pub fn exists(&self) -> bool {
        self.dist2 >= S::zero()
    }
}

impl<S: BaseFloat> PartialEq for Intersection<S> {
    fn eq(&self, other: &Intersection<S>) -> bool {
        self.exists() && other.exists() && self.dist2 == other.dist2
    }
}

impl<S: BaseFloat> PartialOrd for Intersection<S> {
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

pub trait Shape<S: BaseFloat> {
    // Returns negative value if there is no intersection, or the square distance to
    // the intersection if there is one.
    fn ray_intersect(&self, origin: Point3<S>, dir: Vector3<S>) -> Intersection<S>;
}

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
        let to_center = self.center - origin;
        let p = dot(to_center, dir);
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

#[derive(Clone, Copy)]
pub struct Material<S: BaseFloat> {
    pub color: image::Rgb<u8>,
    pub diffusion: S,
    pub reflection: S,
    pub shininess: S,
}

impl Material<f32> {
    pub fn new(r: f32, g: f32, b: f32) -> Self {
        Material {
            color: image::Rgb([(r * 255.) as u8, (g * 255.) as u8, (b * 255.) as u8]),
            diffusion: 0.7,
            reflection: 3.0,
            shininess: 10.0,
        }
    }
}

pub struct PointLight<S: BaseFloat> {
    position: Point3<S>,
    intensity: S,
}

pub struct Scene<S: BaseFloat> {
    spheres: Vec<(usize, Sphere<S>)>,
    planes: Vec<(usize, Plane<S>)>,
    materials: Vec<Material<S>>,
    lights: Vec<PointLight<S>>,
}

impl<S: BaseFloat> Scene<S> {
    pub fn new() -> Self {
        Scene {
            spheres: Vec::new(),
            planes: Vec::new(),
            materials: Vec::new(),
            lights: Vec::new(),
        }
    }

    pub fn add_sphere(&mut self, sphere: Sphere<S>, material: Material<S>) -> usize {
        let id = self.materials.len();
        self.spheres.push((id, sphere));
        self.materials.push(material);
        id
    }

    pub fn add_plane(&mut self, plane: Plane<S>, material: Material<S>) -> usize {
        let id = self.materials.len();
        self.planes.push((id, plane));
        self.materials.push(material);
        id
    }

    pub fn add_light(&mut self, position: Point3<S>, intensity: S) {
        self.lights.push(PointLight {
            position,
            intensity,
        })
    }

    pub fn find_intersection(
        &self,
        origin: Point3<S>,
        dir: Vector3<S>,
    ) -> (Intersection<S>, usize) {
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
}

impl<S: BaseFloat + AsPrimitive<f32>> Scene<S> {
    pub fn ray_color(&self, origin: Point3<S>, dir: Vector3<S>) -> image::Rgb<u8> {
        let (intersection, id) = self.find_intersection(origin, dir);
        if !intersection.exists() {
            return image::Rgb([0, 0, 0]);
        }

        let material = self.materials[id];
        let ipoint = origin + dir * (intersection.dist2 / dir.magnitude2()).sqrt();
        let dir = dir.normalize();
        let mut illumination = S::zero();
        let normal = intersection.normal.normalize();

        for light in self.lights.iter() {
            let light_vec = light.position - ipoint;
            let light_dir = light_vec.normalize();
            let (to_light_int, _) = self.find_intersection(ipoint, light_vec);
            if to_light_int.exists() {
                continue;
            }
            let dist2 = light_vec.magnitude2();
            let diffusion_intensity = dot(normal, light_vec);
            // Light is on the other side of the surface.
            if diffusion_intensity <= S::default_epsilon() {
                continue;
            }

            let reflect_vec = normal * (S::one() + S::one()) * dot(light_dir, normal) - light_dir;
            let reflect_vec = reflect_vec.normalize();
            let reflect_intensity = dot(reflect_vec, -dir);
            let reflect_intensity = if reflect_intensity > S::zero() {
                reflect_intensity.powf(material.shininess)
            } else {
                S::zero()
            };

            // dbg!(material.diffusion);
            // assert_eq!(S::zero(), material.reflection * reflect_intensity);

            let intensity1 = light.intensity / dist2 * diffusion_intensity;
            let intensity2 = light.intensity / dist2
                * (material.diffusion * diffusion_intensity
                    + material.reflection * reflect_intensity);
            illumination = illumination + intensity2;
        }

        image::Rgb([
            (material.color[0] as f32 * illumination.as_() as f32).min(255.) as u8,
            (material.color[1] as f32 * illumination.as_() as f32).min(255.) as u8,
            (material.color[2] as f32 * illumination.as_() as f32).min(255.) as u8,
        ])
    }
}

pub struct Camera<S: BaseFloat> {
    w: u32,
    h: u32,
    scale: S,
    origin: Point3<S>,
}

impl Camera<f32> {
    // `fov` -- vertical field of view, horizontal field of view scales with
    pub fn new(w: u32, h: u32, origin: Point3<f32>) -> Self {
        Camera {
            w,
            h,
            origin,
            scale: 2. / (h as f32),
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
            let dir = Point3::new(x, y, 0.) - self.origin;
            *pixel = scene.ray_color(self.origin, dir);
        }

        let t = time::Instant::now() - start;
        let t = t.as_secs() as f64 + 1E-9 * t.subsec_nanos() as f64;
        println!("Elapsed {} ms", t * 1000.);
        println!("{} ns per ray", (t / rays as f64) * 1E9);
        image
    }
}
