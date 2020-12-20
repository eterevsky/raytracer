use criterion::Criterion;
use criterion::{black_box, criterion_group, criterion_main};
use rand::SeedableRng as _;
use glam::{Vec3, vec3};

use raytracer::*;

fn sphere_ray(c: &mut Criterion) {
    let sphere = Sphere::new(vec3(0.1, 0.2, 3.), 1.);
    let origin = Vec3::zero();
    let dir = Vec3::unit_z();
    c.bench_function("sphere ray", |b| {
        b.iter(|| black_box(&sphere).ray_intersect(black_box(origin), black_box(dir)))
    });
}

fn plane_ray(c: &mut Criterion) {
    let plane = Plane::new(vec3(0., -1., 0.), Vec3::unit_y());
    let origin = Vec3::new(0., 0., 3.);
    let dir = Vec3::new(0., -0.1, 1.).normalize();
    c.bench_function("plane ray", |b| {
        b.iter(|| black_box(&plane).ray_intersect(black_box(origin), black_box(dir)))
    });
}

fn create_scene() -> Scene {
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

    scene
}

fn scene_ray(c: &mut Criterion) {
    let scene = create_scene();
    let origin = vec3(0., 0., 3.);
    let dir = vec3(0., 0., 1.);
    c.bench_function("scene ray", |b| {
        b.iter(|| black_box(&scene).find_intersection(black_box(origin), black_box(dir)))
    });
}

fn transform_ray(c: &mut Criterion) {
    let camera = Camera::new()
        .set_eye(Vec3::new(1., 2., 3.))
        .set_target(Vec3::zero());
    let vector = Vec3::new(0.1, 0.2, -1.0);

    c.bench_function("transform_ray", |b| {
        b.iter(|| black_box(&camera).transform_ray(black_box(vector)))
    });
}

fn pixel_ray(c: &mut Criterion) {
    let camera = Camera::new()
        .set_eye(vec3(1., 2., 3.))
        .set_target(Vec3::zero());

    c.bench_function("pixel_ray", |b| {
        b.iter(|| black_box(&camera).pixel_ray(black_box(123), black_box(345)))
    });
}

fn sample_pixel_ray_smallrng(c: &mut Criterion) {
    let camera = Camera::new()
        .set_eye(vec3(1., 2., 3.))
        .set_target(Vec3::zero());

    let mut rng = rand::rngs::SmallRng::seed_from_u64(239);

    c.bench_function("sample_pixel_ray_smallrng", |b| {
        b.iter(|| black_box(&camera).sample_pixel_ray(
            black_box(123), black_box(345), &mut rng))
    });
}

fn sample_pixel_ray_threadrng(c: &mut Criterion) {
    let camera = Camera::new()
        .set_eye(vec3(1., 2., 3.))
        .set_target(Vec3::zero());

    let mut rng = rand::thread_rng();

    c.bench_function("sample_pixel_ray_threadrng", |b| {
        b.iter(|| black_box(&camera).sample_pixel_ray(
            black_box(123), black_box(345), &mut rng))
    });
}

fn render256x256_empty_smallrng(c: &mut Criterion) {
    let scene = Scene::new();
    let camera = Camera::new().set_dimensions(256, 256).set_samples(1);
    let mut rng = rand::rngs::SmallRng::seed_from_u64(239);

    c.bench_function("render256x256_empty_smallrng_glam", |b| {
        b.iter(|| black_box(&camera).render(black_box(&scene), &mut rng))
    });
}

fn render256x256_empty_threadrng(c: &mut Criterion) {
    let scene = Scene::new();
    let camera = Camera::new().set_dimensions(256, 256).set_samples(1);
    let mut rng = rand::thread_rng();

    c.bench_function("render256x256_empty_threadrng", |b| {
        b.iter(|| black_box(&camera).render(black_box(&scene), &mut rng))
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

fn render16x16_threadrng(c: &mut Criterion) {
    let scene = create_scene();

    let camera = Camera::new()
        .set_dimensions(16, 16)
        .set_fov(std::f32::consts::PI / 5.);

    let mut rng = rand::thread_rng();

    c.bench_function("render16x16_threadrng", |b| {
        b.iter(|| black_box(&camera).render(black_box(&scene), &mut rng))
    });
}

criterion_group!{
    name = benches;
    config = Criterion::default().significance_level(0.05).sample_size(500);
    targets = sphere_ray,
    plane_ray,
    scene_ray,
    transform_ray,
    pixel_ray,
    sample_pixel_ray_smallrng,
    sample_pixel_ray_threadrng,
    render256x256_empty_smallrng,
    render256x256_empty_threadrng,
    render16x16_smallrng,
    render16x16_threadrng,
}

criterion_main!(benches);
