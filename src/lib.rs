#[cfg(test)]
#[macro_use]
extern crate approx;

mod camera;
mod light;
mod material;
mod plane;
mod scene;
mod shape;
mod sphere;

pub use self::camera::Camera;
pub use self::scene::Scene;
pub use self::plane::Plane;
pub use self::sphere::Sphere;
pub use self::material::Material;
pub use self::shape::Shape;