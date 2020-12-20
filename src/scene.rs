use glam::Vec3;

use crate::defines::*;
use crate::light::{Light, PointLight, SphereLight};
use crate::material::{Color, Material};
use crate::plane::Plane;
use crate::shape::{Intersection, Shape};
use crate::sphere::Sphere;

pub struct Scene {
    spheres: Vec<(usize, Sphere)>,
    planes: Vec<(usize, Plane)>,
    materials: Vec<Material>,
    point_lights: Vec<PointLight>,
    sphere_lights: Vec<SphereLight>,
}

impl Scene {
    pub fn new() -> Self {
        Scene {
            spheres: Vec::new(),
            planes: Vec::new(),
            materials: Vec::new(),
            point_lights: Vec::new(),
            sphere_lights: Vec::new(),
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

    pub fn add_point_light(&mut self, position: Vec3, intensity: f32) {
        self.point_lights.push(PointLight::new(position, intensity));
    }

    pub fn add_sphere_light(&mut self, center: Vec3, radius: f32, intensity: f32) {
        self.sphere_lights
            .push(SphereLight::new(center, radius, intensity))
    }

    pub fn find_intersection(&self, origin: Vec3, dir: Vec3) -> (Intersection, usize) {
        let mut best_idx = 0;
        let mut nearest = Intersection::new_empty();

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

    // dir: direction of the ray from the camera to the surface,
    // normal: normal to the surface
    fn illumination_from_light(
        &self,
        point: Vec3,
        normal: Vec3,
        dir: Vec3,
        material: &Material,
        light: &impl Light,
        rng: &mut impl rand::Rng,
    ) -> f32 {
        let light_vec = light.sample_ray(point, rng);
        let light_dist2 = light_vec.length_squared();
        let light_dist = light_dist2.sqrt();
        let light_dir = light_vec / light_dist;

        let expanded = point + normal * EPSILON;
        let (to_light_int, _) = self.find_intersection(expanded, light_dir);
        if to_light_int.exists() && to_light_int.dist < light_dist {
            return 0.;
        }
        let diffusion_intensity = normal.dot(light_dir);
        // Light is on the other side of the surface.
        if diffusion_intensity < EPSILON {
            // TODO: shouldn't happen
            return 0.;
        }

        let reflect_vec = normal * (2. * light_dir.dot(normal)) - light_dir;
        let reflect_vec = reflect_vec.normalize();
        let reflect_intensity = reflect_vec.dot(-dir);
        let reflect_intensity = if reflect_intensity > 0. {
            reflect_intensity.powf(material.shininess)
        } else {
            0.
        };

        light.intensity() / light_dist2
            * (material.diffusion * diffusion_intensity
                + material.reflection * reflect_intensity)
    }

    pub fn ray_color(&self, origin: Vec3, dir: Vec3, rng: &mut impl rand::Rng)
    -> Color {
        let (intersection, id) = self.find_intersection(origin, dir);
        if !intersection.exists() {
            return Color::black();
        }

        let material = self.materials[id];
        let ipoint = origin + dir * intersection.dist;
        let mut illumination = 0.;
        let normal = intersection.normal;

        for light in self.point_lights.iter() {
            illumination += self.illumination_from_light(
                ipoint, normal, dir, &material, light, rng);
        }

        for light in self.sphere_lights.iter() {
            illumination += self.illumination_from_light(
                ipoint,
                normal,
                dir,
                &material,
                light,
                rng,
            );
        }

        material.color * illumination
    }
}
