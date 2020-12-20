use crate::shape::*;
use glam::Vec3;

pub struct Sphere {
    center: Vec3,
    radius: f32,
    radius2: f32,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32) -> Self {
        Sphere {
            center,
            radius,
            radius2: radius * radius,
        }
    }
}

impl Shape for Sphere {
    fn ray_intersect(&self, origin: Vec3, dir: Vec3) -> Intersection {
        let to_center = self.center - origin;
        // Projection of the line to the sphere center on to the ray.
        let projection = dir.dot(to_center);
        if projection <= 0. {
            return Intersection::new_empty();
        }

        let projection2 = projection * projection;
        let ray_dist2 = to_center.length_squared() - projection2;
        if ray_dist2 >= self.radius2 {
            return Intersection::new_empty();
        }

        let seg2 = self.radius2 - ray_dist2;
        if projection2 <= seg2 {
            return Intersection::new_empty();
        }
        let dist = projection2.sqrt() - seg2.sqrt();
        let to_intersect = dir * dist;
        let normal = (to_intersect - to_center) / self.radius;
        Intersection::new(dist, normal)
    }
}

#[cfg(test)]
mod tests {
    use glam::vec3;

    use super::*;

    #[test]
    fn sphere_ray_intersect1() {
        let sphere = Sphere::new(vec3(0., 0., 2.), 1.);
        let intersection = sphere.ray_intersect(vec3(0., 0., 0.), vec3(0., 0., 1.));
        assert_eq!(intersection.dist, 1.)
    }

    #[test]
    fn sphere_ray_intersect2() {
        let sphere = Sphere::new(vec3(0., 0., 3.), 1.);
        let intersection = sphere.ray_intersect(vec3(0., 0., 0.), vec3(0., 0., 1.));
        assert_eq!(intersection.dist, 2.)
    }

    #[test]
    fn sphere_ray_intersect3() {
        let sphere = Sphere::new(vec3(0., 0., 2.), 1.);
        let intersection = sphere.ray_intersect(vec3(0., 0., 0.), vec3(0., 0., 1.));
        assert_eq!(intersection.dist, 1.)
    }

    #[test]
    fn sphere_ray_intersect4() {
        let sphere = Sphere::new(vec3(0., 0., 3.), 1.);
        let intersection = sphere.ray_intersect(vec3(0., 0., 0.), vec3(0., 0., 1.));
        assert_eq!(intersection.dist, 2.)
    }

    #[test]
    fn sphere_ray_intersect5() {
        let sphere = Sphere::new(vec3(0., 0., 3.), 1.);
        let intersection1 =
            sphere.ray_intersect(vec3(0., 0., 0.), vec3(0., -1., 3.).normalize());
        let intersection2 =
            sphere.ray_intersect(vec3(0., 0., 0.), vec3(0., -0.5, 1.5).normalize());
        assert_eq!(intersection1.dist, intersection2.dist);
        assert!(intersection1.dist > 2.5);
        assert!(intersection1.dist < 2.8);
    }
}
