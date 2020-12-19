use cgmath::{dot, InnerSpace, Point3, Vector3};
use image;
use rand;

use crate::light::{Light, PointLight, SphereLight};
use crate::material::Material;
use crate::plane::Plane;
use crate::shape::{Intersection, Shape};
use crate::sphere::Sphere;

pub struct Scene {
    spheres: Vec<(usize, Sphere)>,
    planes: Vec<(usize, Plane)>,
    materials: Vec<Material>,
    point_lights: Vec<PointLight>,
    sphere_lights: Vec<SphereLight>,
    sphere_light_samples: u32,
}

impl Scene {
    pub fn new() -> Self {
        Scene {
            spheres: Vec::new(),
            planes: Vec::new(),
            materials: Vec::new(),
            point_lights: Vec::new(),
            sphere_lights: Vec::new(),
            sphere_light_samples: 100,
        }
    }

    pub fn add_sphere(&mut self, sphere: Sphere, material: Material) -> usize {
        let id = self.materials.len();
        self.spheres.push((id, sphere));
        self.materials.push(material);
        id
    }

    pub fn add_plane(&mut self, plane: Plane, material: Material) -> usize {
        let id = self.materials.len();
        self.planes.push((id, plane));
        self.materials.push(material);
        id
    }

    pub fn add_point_light(&mut self, position: Point3<f32>, intensity: f32) {
        self.point_lights.push(PointLight::new(position, intensity));
    }

    pub fn add_sphere_light(&mut self, center: Point3<f32>, radius: f32, intensity: f32) {
        self.sphere_lights
            .push(SphereLight::new(center, radius, intensity))
    }

    pub fn set_sphere_light_samples(mut self, value: u32) -> Self {
        self.sphere_light_samples = value;
        self
    }

    pub fn find_intersection(
        &self,
        origin: Point3<f32>,
        dir: Vector3<f32>,
    ) -> (Intersection, usize) {
        let mut best_idx = 0;
        let mut nearest = Intersection::no();

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

    fn illumination_from_light<L: Light, R: rand::Rng>(
        &self,
        rng: &mut R,
        point: Point3<f32>,
        normal: Vector3<f32>,
        dir: Vector3<f32>,
        material: &Material,
        light: &L,
        samples: u32,
    ) -> f32 {
        let mut total = 0.;
        for _ in 0..samples {
            let light_vec = light.sample_ray(point, rng);
            let light_dist2 = light_vec.magnitude2();
            let light_dir = light_vec.normalize();
            let expanded = point + normal * 0.001;
            let (to_light_int, _) = self.find_intersection(expanded, light_dir);
            if to_light_int.exists() && to_light_int.dist2 < light_dist2 {
                continue;
            }
            let dist2 = light_vec.magnitude2();
            let diffusion_intensity = dot(normal, light_dir);
            // Light is on the other side of the surface.
            if diffusion_intensity <= 1E-6 {
                continue;
            }

            let reflect_vec = normal * 2. * dot(light_dir, normal) - light_dir;
            let reflect_vec = reflect_vec.normalize();
            let reflect_intensity = dot(reflect_vec, -dir);
            let reflect_intensity = if reflect_intensity > 0. {
                reflect_intensity.powf(material.shininess)
            } else {
                0.
            };

            total += light.intensity() / dist2
                * (material.diffusion * diffusion_intensity
                    + material.reflection * reflect_intensity)
        }

        total / samples as f32
    }

    pub fn ray_color<R: rand::Rng>(&self, rng: &mut R, origin: Point3<f32>, dir: Vector3<f32>) -> image::Rgb<u8> {
        let (intersection, id) = self.find_intersection(origin, dir);
        if !intersection.exists() {
            return image::Rgb([0, 0, 0]);
        }

        let material = self.materials[id];
        let ipoint = origin + dir * (intersection.dist2 / dir.magnitude2()).sqrt();
        let dir = dir.normalize();
        let mut illumination = 0.;
        let normal = intersection.normal.normalize();

        for light in self.point_lights.iter() {
            illumination += self.illumination_from_light(
                rng, ipoint, normal, dir, &material, light, 1);
        }

        for light in self.sphere_lights.iter() {
            illumination += self.illumination_from_light(
                rng,
                ipoint,
                normal,
                dir,
                &material,
                light,
                self.sphere_light_samples,
            );
        }

        image::Rgb([
            ((material.color[0] * illumination).min(1.) * 255.) as u8,
            ((material.color[1] * illumination).min(1.) * 255.) as u8,
            ((material.color[2] * illumination).min(1.) * 255.) as u8,
        ])
    }
}
