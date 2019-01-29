use rt::*;

use criterion::Criterion;
use criterion::{criterion_group, criterion_main};

fn sphere_ray(c: &mut Criterion) {
    c.bench_function(
        "sphere ray",
        |b| {
            let sphere = Sphere::new(Pnt3(0.1, 0.2, 3.), 1.);
            let origin = Pnt3(0., 0., 0.);
            let dir = Vec3(0., 0., 1.);
            b.iter(|| sphere.ray_intersect(origin, dir))
        });
}

fn sphere_ray_f64(c: &mut Criterion) {
    c.bench_function(
        "sphere ray f64",
        |b| {
            let sphere = Sphere::new(Pnt3::<f64>(0.1, 0.2, 3.), 1f64);
            let origin = Pnt3::<f64>(0., 0., 0.);
            let dir = Vec3::<f64>(0., 0., 1.);
            b.iter(|| sphere.ray_intersect(origin, dir))
        });
}

criterion_group!(benches, sphere_ray, sphere_ray_f64);
criterion_main!(benches);