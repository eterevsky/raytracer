use cgmath::{Point3, Vector3, Zero};
use std::cmp::{Ordering, PartialOrd};

#[derive(Debug)]
pub struct Intersection {
    pub dist2: f32,
    pub normal: Vector3<f32>,
}

impl Intersection {
    pub fn no() -> Self {
        Intersection {
            dist2: -1.,
            normal: Vector3::zero(),
        }
    }

    pub fn new(dist2: f32, normal: Vector3<f32>) -> Self {
        Intersection { dist2, normal }
    }

    pub fn exists(&self) -> bool {
        self.dist2 >= 0.
    }
}

impl PartialEq for Intersection {
    fn eq(&self, other: &Intersection) -> bool {
        self.exists() && other.exists() && self.dist2 == other.dist2
    }
}

impl PartialOrd for Intersection {
    fn partial_cmp(&self, other: &Intersection) -> Option<Ordering> {
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

pub trait Shape {
    /// Returns negative value if there is no intersection, or the square distance to
    /// the intersection if there is one.
    fn ray_intersect(&self, origin: Point3<f32>, dir: Vector3<f32>) -> Intersection;
}

pub struct IntersectionN {
    /// Square of distance if the intersection exists or a negative value if it doesn't.
    pub dist: f32,
    pub normal: nalgebra::Unit<nalgebra::Vector3<f32>>,
}

impl IntersectionN {
    pub fn new(dist: f32, normal: nalgebra::Unit<nalgebra::Vector3<f32>>) -> Self {
        IntersectionN { dist, normal }
    }

    pub fn new_empty() -> Self {
        IntersectionN {
            dist: -1.,
            normal: nalgebra::Vector3::x_axis(),
        }
    }
}

pub trait ShapeN {
    /// Returns negative value if there is no intersection, or the square distance to
    /// the intersection if there is one.
    fn ray_intersect(
        &self,
        origin: nalgebra::Point3<f32>,
        dir: nalgebra::Unit<nalgebra::Vector3<f32>>,
    ) -> IntersectionN;
}
