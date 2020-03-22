use cgmath;
use cgmath::num_traits::AsPrimitive;
use cgmath::{dot, BaseFloat, InnerSpace, Point3, Vector3};
use image;
use rand;
use std::time;

mod light;
mod plane;
mod shape;
mod sphere;

pub use self::light::{Light, PointLight, SphereLight};
pub use self::plane::Plane;
pub use self::shape::{Intersection, Shape};
pub use self::sphere::Sphere;


#[derive(Clone, Copy)]
pub struct Material<S: BaseFloat> {
    pub color: image::Rgb<f32>,
    pub diffusion: S,
    pub reflection: S,
    pub shininess: S,
    pub mirror: bool,
}

impl<S: BaseFloat + AsPrimitive<f32>> Material<S> {
    pub fn new(r: S, g: S, b: S) -> Self {
        Material {
            color: image::Rgb([r.as_() as f32, g.as_() as f32, b.as_() as f32]),
            diffusion: S::from(1.0).unwrap(),
            reflection: S::from(3.0).unwrap(),
            shininess: S::from(10.0).unwrap(),
            mirror: false,
        }
    }
}

pub struct Scene<S: BaseFloat> {
    spheres: Vec<(usize, Sphere<S>)>,
    planes: Vec<(usize, Plane<S>)>,
    materials: Vec<Material<S>>,
    point_lights: Vec<PointLight<S>>,
    sphere_lights: Vec<SphereLight<S>>,
}

impl<S: BaseFloat> Scene<S> {
    pub fn new() -> Self {
        Scene {
            spheres: Vec::new(),
            planes: Vec::new(),
            materials: Vec::new(),
            point_lights: Vec::new(),
            sphere_lights: Vec::new(),
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

    pub fn add_point_light(&mut self, position: Point3<S>, intensity: S) {
        self.point_lights.push(PointLight::new(position, intensity));
    }

    pub fn add_sphere_light(&mut self, center: Point3<S>, radius: S, intensity: S) {
        self.sphere_lights
            .push(SphereLight::new(center, radius, intensity))
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
    fn illumination_from_light<L: Light<S>>(
        &self,
        point: Point3<S>,
        normal: Vector3<S>,
        dir: Vector3<S>,
        material: &Material<S>,
        light: &L,
        samples: u32,
    ) -> S {
        let mut total = S::zero();
        let mut rng = rand::thread_rng();
        for _ in 0..samples {
            let light_vec = light.sample_ray(point, &mut rng);
            let light_dist2 = light_vec.magnitude2();
            let light_dir = light_vec.normalize();
            let expanded = point + normal * S::from(0.001).unwrap();
            let (to_light_int, _) = self.find_intersection(expanded, light_dir);
            if to_light_int.exists() && to_light_int.dist2 < light_dist2 {
                continue;
            }
            let dist2 = light_vec.magnitude2();
            let diffusion_intensity = dot(normal, light_dir);
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

            total += light.intensity() / dist2
                * (material.diffusion * diffusion_intensity
                    + material.reflection * reflect_intensity)
        }

        total / S::from(samples).unwrap()
    }

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

        for light in self.point_lights.iter() {
            illumination += self.illumination_from_light(ipoint, normal, dir, &material, light, 1);
        }

        for light in self.sphere_lights.iter() {
            illumination += self.illumination_from_light(ipoint, normal, dir, &material, light, 100);
        }

        image::Rgb([
            ((material.color[0] * illumination.as_() as f32).min(1.) * 255.) as u8,
            ((material.color[1] * illumination.as_() as f32).min(1.) * 255.) as u8,
            ((material.color[2] * illumination.as_() as f32).min(1.) * 255.) as u8,
        ])
    }
}

pub struct Camera<S: BaseFloat> {
    w: u32,
    h: u32,
    scale: S,
    origin: Point3<S>,
}

impl<S: BaseFloat + AsPrimitive<f32>> Camera<S> {
    pub fn new(w: u32, h: u32, origin: Point3<S>) -> Self {
        Camera {
            w,
            h,
            origin,
            scale: (S::one() + S::one()) / S::from(h).unwrap(),
        }
    }

    pub fn render(&self, scene: &Scene<S>) -> image::ImageBuffer<image::Rgb<u8>, Vec<u8>> {
        let mut image = image::ImageBuffer::new(self.w, self.h);
        let start = time::Instant::now();
        let mut rays: u64 = 0;

        for (x, y, pixel) in image.enumerate_pixels_mut() {
            rays += 1;
            let x = S::from(x).unwrap() * self.scale - S::one();
            let y = -S::from(y).unwrap() * self.scale + S::one();
            let dir = Point3::new(x, y, S::zero()) - self.origin;
            let dir = dir.normalize();
            *pixel = scene.ray_color(self.origin, dir);
        }

        let t = time::Instant::now() - start;
        let t = t.as_secs() as f64 + 1E-9 * t.subsec_nanos() as f64;
        println!("Elapsed {} ms", t * 1000.);
        println!("{} ns per ray", (t / rays as f64) * 1E9);
        image
    }
}
