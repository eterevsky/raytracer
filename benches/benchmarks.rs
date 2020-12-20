use cgmath::{Point3, Vector3};
use criterion::Criterion;
use criterion::{black_box, criterion_group, criterion_main};
use rand::SeedableRng as _;
use glam::{Vec3, Vec3A};

use raytracer::*;

fn sphere_ray(c: &mut Criterion) {
    let sphere = Sphere::new(Point3::new(0.1, 0.2, 3.), 1.);
    let origin = Point3::new(0., 0., 0.);
    let dir = Vector3::new(0., 0., 1.);
    c.bench_function("sphere ray", |b| {
        b.iter(|| black_box(&sphere).ray_intersect(black_box(origin), black_box(dir)))
    });
}

fn spheren_ray(c: &mut Criterion) {
    let sphere = SphereN::new(nalgebra::Point3::new(0.1, 0.2, 3.), 1.);
    let origin = nalgebra::Point3::origin();
    let dir = nalgebra::Vector3::z_axis();
    c.bench_function("sphere ray (nalgebra)", |b| {
        b.iter(|| black_box(&sphere).ray_intersect(black_box(origin), black_box(dir)))
    });
}

fn sphereg_ray(c: &mut Criterion) {
    let sphere = SphereG::new(Vec3::new(0.1, 0.2, 3.), 1.);
    let origin = Vec3::zero();
    let dir = Vec3::unit_z();
    c.bench_function("sphere ray (glam)", |b| {
        b.iter(|| black_box(&sphere).ray_intersect(black_box(origin), black_box(dir)))
    });
}

fn spherega_ray(c: &mut Criterion) {
    let sphere = SphereGA::new(Vec3A::new(0.1, 0.2, 3.), 1.);
    let origin = Vec3A::zero();
    let dir = Vec3A::unit_z();
    c.bench_function("sphere ray (glam w/ alignment)", |b| {
        b.iter(|| black_box(&sphere).ray_intersect(black_box(origin), black_box(dir)))
    });
}

fn plane_ray(c: &mut Criterion) {
    let plane = Plane::new(Point3::new(0., -1., 0.), Vector3::new(0., 1., 0.));
    let origin = Point3::new(0., 0., 3.);
    let dir = Vector3::new(0., -0.1, 1.);
    c.bench_function("plane ray", |b| {
        b.iter(|| black_box(&plane).ray_intersect(black_box(origin), black_box(dir)))
    });
}

fn planen_ray(c: &mut Criterion) {
    let plane = PlaneN::new(nalgebra::Point3::new(0., -1., 0.), nalgebra::Vector3::y_axis());
    let origin = nalgebra::Point3::new(0., 0., 3.);
    let dir = nalgebra::Unit::new_normalize(nalgebra::Vector3::new(0., -0.1, 1.));
    c.bench_function("plane ray (nalgebra)", |b| {
        b.iter(|| black_box(&plane).ray_intersect(black_box(origin), black_box(dir)))
    });
}

fn planeg_ray(c: &mut Criterion) {
    let plane = PlaneG::new(Vec3::new(0., -1., 0.), Vec3::unit_y());
    let origin = Vec3::new(0., 0., 3.);
    let dir = Vec3::new(0., -0.1, 1.).normalize();
    c.bench_function("plane ray (glam)", |b| {
        b.iter(|| black_box(&plane).ray_intersect(black_box(origin), black_box(dir)))
    });
}

fn planega_ray(c: &mut Criterion) {
    let plane = PlaneGA::new(Vec3A::new(0., -1., 0.), Vec3A::unit_y());
    let origin = Vec3A::new(0., 0., 3.);
    let dir = Vec3A::new(0., -0.1, 1.).normalize();
    c.bench_function("plane ray (glam w/ alignment)", |b| {
        b.iter(|| black_box(&plane).ray_intersect(black_box(origin), black_box(dir)))
    });
}

fn create_scene() -> Scene {
    let mut scene = Scene::new().set_sphere_light_samples(100);
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

    scene
}

fn scene_ray(c: &mut Criterion) {
    let scene = create_scene();
    let origin = Point3::new(0., 0., 3.);
    let dir = Vector3::new(0., 0., 1.);
    c.bench_function("scene ray", |b| {
        b.iter(|| black_box(&scene).find_intersection(black_box(origin), black_box(dir)))
    });
}

fn render16x16_smallrng(c: &mut Criterion) {
    let scene = create_scene();

    let camera = Camera::new()
        .set_dimensions(16, 16)
        .set_fov(std::f32::consts::PI / 5.);

    let mut rng = rand::rngs::SmallRng::seed_from_u64(239);

    c.bench_function("render16x16_smallrng", |b| {
        b.iter(|| black_box(&camera).render(black_box(&scene), &mut rng))
    });
}

fn render16x16_thread_rng(c: &mut Criterion) {
    let scene = create_scene();

    let camera = Camera::new()
        .set_dimensions(16, 16)
        .set_fov(std::f32::consts::PI / 5.);

    let mut rng = rand::thread_rng();

    c.bench_function("render16x16_threadrng", |b| {
        b.iter(|| black_box(&camera).render(black_box(&scene), &mut rng))
    });
}

fn transform_ray(c: &mut Criterion) {
    let camera = Camera::new()
        .set_eye(nalgebra::Point3::new(1., 2., 3.))
        .set_target(nalgebra::Point3::origin());
    let vector = nalgebra::Vector3::new(0.1, 0.2, -1.0);

    c.bench_function("transform_ray", |b| {
        b.iter(|| black_box(&camera).transform_ray(black_box(&vector)))
    });
}

fn pixel_ray(c: &mut Criterion) {
    let camera = Camera::new()
        .set_eye(nalgebra::Point3::new(1., 2., 3.))
        .set_target(nalgebra::Point3::origin());

    c.bench_function("pixel_ray", |b| {
        b.iter(|| black_box(&camera).pixel_ray(black_box(123), black_box(345)))
    });
}

fn sample_pixel_ray_small(c: &mut Criterion) {
    let camera = Camera::new()
        .set_eye(nalgebra::Point3::new(1., 2., 3.))
        .set_target(nalgebra::Point3::origin());

    let mut rng = rand::rngs::SmallRng::seed_from_u64(239);

    c.bench_function("sample_pixel_ray_smallrng", |b| {
        b.iter(|| black_box(&camera).sample_pixel_ray(
            black_box(123), black_box(345), &mut rng))
    });
}

fn sample_pixel_ray_thread_rng(c: &mut Criterion) {
    let camera = Camera::new()
        .set_eye(nalgebra::Point3::new(1., 2., 3.))
        .set_target(nalgebra::Point3::origin());

    let mut rng = rand::thread_rng();

    c.bench_function("sample_pixel_ray_threadrng", |b| {
        b.iter(|| black_box(&camera).sample_pixel_ray(
            black_box(123), black_box(345), &mut rng))
    });
}

fn render256x256_empty_smallrng(c: &mut Criterion) {
    let scene = Scene::new().set_sphere_light_samples(1);
    let camera = Camera::new().set_dimensions(256, 256);
    let mut rng = rand::rngs::SmallRng::seed_from_u64(239);

    c.bench_function("render256x256_empty_smallrng", |b| {
        b.iter(|| black_box(&camera).render(black_box(&scene), &mut rng))
    });
}

fn render256x256_empty_thread_rng(c: &mut Criterion) {
    let scene = Scene::new().set_sphere_light_samples(1);
    let camera = Camera::new().set_dimensions(256, 256);
    let mut rng = rand::thread_rng();

    c.bench_function("render256x256_empty_threadrng", |b| {
        b.iter(|| black_box(&camera).render(black_box(&scene), &mut rng))
    });
}

criterion_group!{
    name = benches;
    config = Criterion::default().significance_level(0.05).sample_size(500);
    targets = sphere_ray,
    spheren_ray,
    sphereg_ray,
    spherega_ray,
    plane_ray,
    planen_ray,
    planeg_ray,
    planega_ray,
    scene_ray,
    render16x16_smallrng,
    render16x16_thread_rng,
    transform_ray,
    pixel_ray,
    sample_pixel_ray_small,
    sample_pixel_ray_thread_rng,
    render256x256_empty_smallrng,
    render256x256_empty_thread_rng,
}

criterion_main!(benches);
