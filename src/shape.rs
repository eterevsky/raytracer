use glam::Vec3;
use std::cmp::{Ordering, PartialOrd};

use crate::defines::*;

#[derive(Debug)]
pub struct Intersection {
    pub dist: f32,
    pub normal: Vec3,
}

impl Intersection {
    /// normal should be normalized.
    pub fn new(dist: f32, normal: Vec3) -> Self {
        debug_assert!((normal.length() - 1.).abs() < EPSILON );
        Intersection { dist, normal }
    }

    pub fn new_empty() -> Self {
        Intersection {
            dist: -1.,
            normal: Vec3::unit_x(),
        }
    }

    pub fn exists(&self) -> bool {
        self.dist > 0.
    }
}

impl PartialEq for Intersection {
    fn eq(&self, other: &Intersection) -> bool {
        self.dist == other.dist && self.dist > 0.
    }
}

impl PartialOrd for Intersection {
    fn partial_cmp(&self, other: &Intersection) -> Option<Ordering> {
        if self.exists() {
            if other.exists() {
                self.dist.partial_cmp(&other.dist)
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
    fn ray_intersect(&self, origin: Vec3, dir: Vec3) -> Intersection;
}
