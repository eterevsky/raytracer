use cgmath::{BaseFloat, Point3, Vector3, Zero};
use std::cmp::{Ordering, PartialOrd};

#[derive(Debug)]
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

