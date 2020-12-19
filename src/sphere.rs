use crate::shape::{Intersection, IntersectionN, Shape, ShapeN};
use cgmath::{dot, InnerSpace, Point3, Vector3};

pub struct Sphere {
    center: Point3<f32>,
    radius: f32,
}

impl Sphere {
    pub fn new(center: Point3<f32>, radius: f32) -> Self {
        Sphere { center, radius }
    }
}

impl Shape for Sphere {
    fn ray_intersect(&self, origin: Point3<f32>, dir: Vector3<f32>) -> Intersection {
        // assert_relative_eq!(dir.magnitude2(), S::one());
        let to_center = self.center - origin;
        let p = dot(to_center, dir);
        if p < 0. {
            return Intersection::no();
        }
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
        let dist2 = projection2 + seg2 - (4. * projection2 * seg2).sqrt();
        if dist2 < 1E-6 {
            return Intersection::no();
        }
        let to_intersect = dir * (dist2 / dir.magnitude2()).sqrt();
        Intersection::new(dist2, to_intersect - to_center)
    }
}

pub struct SphereN {
    center: nalgebra::Point3<f32>,
    radius: f32,
    radius2: f32,
}

impl SphereN {
    pub fn new(center: nalgebra::Point3<f32>, radius: f32) -> Self {
        SphereN {
            center,
            radius,
            radius2: radius * radius,
        }
    }
}

impl ShapeN for SphereN {
    fn ray_intersect(
        &self,
        origin: nalgebra::Point3<f32>,
        dir: nalgebra::Unit<nalgebra::Vector3<f32>>,
    ) -> IntersectionN {
        let to_center = self.center - origin;
        // Projection of the line to the sphere center on to the ray.
        let projection = dir.dot(&to_center);
        if projection <= 0. {
            return IntersectionN::new_empty();
        }

        let projection2 = projection * projection;
        let ray_dist2 = to_center.norm_squared() - projection2;
        if ray_dist2 >= self.radius2 {
            return IntersectionN::new_empty();
        }

        let seg2 = self.radius2 - ray_dist2;
        if projection2 <= seg2 {
            return IntersectionN::new_empty();
        }
        let dist = projection2.sqrt() - seg2.sqrt();
        let to_intersect = dir.into_inner() * dist;
        let normal = nalgebra::Unit::new_unchecked((to_intersect - to_center) / self.radius);
        IntersectionN::new(dist, normal)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
        let intersection1 =
            sphere.ray_intersect(Point3::new(0., 0., 0.), Vector3::new(0., -1., 3.));
        let intersection2 =
            sphere.ray_intersect(Point3::new(0., 0., 0.), Vector3::new(0., -0.5, 1.5));
        assert_eq!(intersection1.dist2, intersection2.dist2);
        assert!(intersection1.dist2 < 6.5);
        assert!(intersection1.dist2 > 6.3);
    }

    #[test]
    fn spheren_ray_intersect1() {
        let sphere = SphereN::new(nalgebra::Point3::new(0., 0., 2.), 1.);
        let intersection = sphere.ray_intersect(nalgebra::Point3::new(0., 0., 0.), nalgebra::Vector3::z_axis());
        assert_eq!(intersection.dist, 1.)
    }

    #[test]
    fn spheren_ray_intersect2() {
        let sphere = SphereN::new(nalgebra::Point3::new(0., 0., 3.), 1.);
        let intersection = sphere.ray_intersect(nalgebra::Point3::new(0., 0., 0.), nalgebra::Vector3::z_axis());
        assert_eq!(intersection.dist, 2.)
    }

    #[test]
    fn spheren_ray_intersect3() {
        let sphere = SphereN::new(nalgebra::Point3::new(0., 0., 2.), 1.);
        let intersection = sphere.ray_intersect(nalgebra::Point3::new(0., 0., 0.), nalgebra::Vector3::z_axis());
        assert_eq!(intersection.dist, 1.)
    }

    #[test]
    fn spheren_ray_intersect4() {
        let sphere = SphereN::new(nalgebra::Point3::new(0., 0., 3.), 1.);
        let intersection = sphere.ray_intersect(nalgebra::Point3::new(0., 0., 0.), nalgebra::Vector3::z_axis());
        assert_eq!(intersection.dist, 2.)
    }

    #[test]
    fn spheren_ray_intersect5() {
        let sphere = SphereN::new(nalgebra::Point3::new(0., 0., 3.), 1.);
        let intersection1 =
            sphere.ray_intersect(nalgebra::Point3::new(0., 0., 0.), nalgebra::Unit::new_normalize(nalgebra::Vector3::new(0., -1., 3.)));
        let intersection2 =
            sphere.ray_intersect(nalgebra::Point3::new(0., 0., 0.), nalgebra::Unit::new_normalize(nalgebra::Vector3::new(0., -0.5, 1.5)));
        assert_eq!(intersection1.dist, intersection2.dist);
        assert!(intersection1.dist < 2.6);
        assert!(intersection1.dist > 2.4);
    }
}
