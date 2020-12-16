use cgmath::{Point3, Vector3, Zero};
use criterion::Criterion;
use criterion::{black_box, criterion_group, criterion_main};

use raytracer::*;
use raytracer::Shape;

fn sphere_ray(c: &mut Criterion) {
    c.bench_function(
        "sphere ray",
        |b| {
            let sphere = Sphere::new(Point3::new(0.1, 0.2, 3.), 1.);
            let origin = Point3::new(0., 0., 0.);
            let dir = Vector3::new(black_box(0.), black_box(0.), black_box(1.));
            b.iter(|| sphere.ray_intersect(origin, dir))
        });
}

// fn sphere_ray_f64(c: &mut Criterion) {
//     c.bench_function(
//         "sphere ray f64",
//         |b| {
//             let sphere = Sphere::new(Point3::new(0.1f64, 0.2f64, 3f64), 1f64);
//             let origin = Point3::new(0f64, 0f64, 0f64);
//             let dir = Vector3::new(0., 0., 1.);
//             b.iter(|| sphere.ray_intersect(origin, dir))
//         });
// }

fn plane_ray(c: &mut Criterion) {
    c.bench_function(
        "plane ray",
        |b| {
            let plane = Plane::new(Point3::new(0., -1., 0.), Vector3::new(0., 1., 0.));
            let origin = Point3::new(0., 0., 3.);
            let dir = Vector3::new(black_box(0.), black_box(-0.1), black_box(1.));
            b.iter(|| plane.ray_intersect(origin, dir))
        });
}

fn scene_ray(c: &mut Criterion) {
    c.bench_function(
        "scene ray",
        |b| {
            let mut scene = Scene::new();
            scene.add_sphere(Sphere::new(Point3::new(0.0, 0.0, -3.), 1.), Material::new(0.75, 0.25, 0.25));
            scene.add_plane(Plane::new(Point3::new(0., -1., 0.), Vector3::new(0., 1., 0.)), Material::new(0.25, 0.25, 0.75));
            scene.add_sphere(Sphere::new(Point3::new(1.0, 3.0, -10.), 2.), Material::new(0.25, 0.65, 0.25));
            let origin = Point3::new(0., 0., 3.);
            let dir = Vector3::new(black_box(0.), black_box(0.), black_box(1.));
            b.iter(|| scene.find_intersection(origin, dir))
        });
}

fn render16x16(c: &mut Criterion) {
    c.bench_function(
        "render 16x16",
        |b| {
            let mut scene = Scene::new();
            scene.add_plane(
                Plane::new(Point3::new(0., -1., 0.), Vector3::new(0., 1., 0.)),
                Material::new(0.8, 0.8, 0.8),
            );

            scene.add_sphere(
                Sphere::new(Point3::new(0.0, 0.0, -3.), 1.),
                Material::new(0.75, 0.25, 0.25),
            );
            scene.add_sphere(
                Sphere::new(Point3::new(1.0, 3.0, -10.), 2.),
                Material::new(0.25, 0.65, 0.25),
            );
            scene.add_sphere(
                Sphere::new(Point3::new(0.65, 0.65, -2.3), 0.1),
                Material::new(0.6, 0.4, 0.2),
            );

            scene.add_point_light(Point3::new(0., 0.1, 3.5), 3.);
            scene.add_sphere_light(Point3::new(2., 1., 0.), 0.5, 3.);
            scene.add_sphere_light(Point3::new(-1., 1., 0.), 0.5, 2.);
            scene.add_sphere_light(Point3::new(0., 10., -5.), 1.0, 30.);

            let camera = Camera::new().set_dimensions(16, 16).set_fov(std::f32::consts::PI / 5.);

            b.iter(|| camera.render(&scene))
        }
    );
}

criterion_group!(benches, sphere_ray, plane_ray, scene_ray, render16x16);
criterion_main!(benches);