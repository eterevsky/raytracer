use glam::vec3;
use rand::SeedableRng as _;

use raytracer::*;

fn main() {
    let mut scene = Scene::new();
    scene.add_plane(
        Plane::new(vec3(0., -1., 0.), vec3(0., 1., 0.)),
        Material::new(0.8, 0.8, 0.8),
    );

    scene.add_sphere(
        Sphere::new(vec3(0.0, 0.0, -3.), 1.),
        Material::new(0.75, 0.25, 0.25),
    );
    scene.add_sphere(
        Sphere::new(vec3(1.0, 3.0, -10.), 2.),
        Material::new(0.25, 0.65, 0.25),
    );
    scene.add_sphere(
        Sphere::new(vec3(0.65, 0.65, -2.3), 0.1),
        Material::new(0.6, 0.4, 0.2),
    );

    scene.add_point_light(vec3(0., 0.1, 3.5), 3.);
    scene.add_sphere_light(vec3(2., 1., 0.), 0.5, 3.);
    scene.add_sphere_light(vec3(-1., 1., 0.), 0.5, 2.);
    scene.add_sphere_light(vec3(0., 10., -5.), 1.0, 30.);
    // scene.add_sphere_light(vec3(-0.65, 0.65, -2.3), 0.1, 0.1);

    let mut rng = rand::rngs::SmallRng::from_entropy();

    let camera = Camera::new().set_dimensions(16, 16);
    let image = camera.render(&scene, &mut rng);

    image.save("image.png").unwrap();
}
