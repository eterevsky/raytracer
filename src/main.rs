use image;
use std::time;

use rt::*;

fn main() {
    let mut image = image::ImageBuffer::new(1024, 1024);

    let mut scene = Scene::new();
    scene.add(Sphere::new(Pnt3(0.0, 0.0, 3.), 1.), Material::new(0.75, 0.25, 0.25));
    scene.add(Plane::new(Pnt3(0., -1., 0.), Vec3(0., 1., 0.)), Material::new(0.25, 0.25, 0.75));
    scene.add(Sphere::new(Pnt3(1.0, 3.0, 10.), 2.), Material::new(0.25, 0.65, 0.25));
    let scene = scene;

    let origin = Pnt3(0., 0., -1.);

    let start = time::Instant::now();
    let mut rays: u64 = 0;

    for (x, y, pixel) in image.enumerate_pixels_mut() {
        rays += 1;
        let x = x as f32 / 512. - 1.;
        let y = -(y as f32 / 512. - 1.);
        let dir = Pnt3(x, y, 1.) - origin;

        if let Some(material) = scene.find_intersection(origin, dir) {
            *pixel = material.color;
        } else {
            *pixel = image::Rgb([128u8, 128u8, 128u8]);
        }
    }

    let t = time::Instant::now() - start;
    let t = t.as_secs() as f64 + 1E-9 * t.subsec_nanos() as f64;
    println!("Elapsed {} ms", t * 1000.);
    println!("{} ns per ray", (t / rays as f64) * 1E9);

    image.save("image.png").unwrap();
}
