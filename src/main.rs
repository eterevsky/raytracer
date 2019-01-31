use image;
use std::time;

use rt::*;

fn main() {
    let mut scene = Scene::new();
    scene.add_plane(Plane::new(Pnt3(0., -1., 0.), Vec3(0., 1., 0.)), Material::new(0.25, 0.25, 0.75));

    scene.add_sphere(Sphere::new(Pnt3(0.0, 0.0, -3.), 1.), Material::new(0.75, 0.25, 0.25));
    scene.add_sphere(Sphere::new(Pnt3(1.0, 3.0, -10.), 2.), Material::new(0.25, 0.65, 0.25));
    scene.add_sphere(Sphere::new(Pnt3(0.65, 0.65, -2.3), 0.1), Material::new(0.6, 0.4, 0.2));

    scene.add_light(Pnt3(2., 1., 0.), 3.);
    scene.add_light(Pnt3(-1., 1., 0.), 2.);
    scene.add_light(Pnt3(0., 10., 0.), 5.);

    let camera = Camera::new(1024, 1024, Pnt3(0., 0., 3.));
    let image = camera.render(&scene);

    image.save("image.png").unwrap();
}
