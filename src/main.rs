use image;
use std::time;

use rt::*;

fn main() {
    let mut image = image::ImageBuffer::new(1024, 1024);

    let sphere = Sphere::new(Pnt3(0.0, 0.0, 3.), 1.);
    let plane = Plane::new(Pnt3(0., -1., 0.), Vec3(0., 1., 0.));
    let origin = Pnt3(0., 0., -1.);

    let start = time::Instant::now();
    let mut rays: u64 = 0;

    for (x, y, pixel) in image.enumerate_pixels_mut() {
        rays += 1;
        let x = x as f32 / 512. - 1.;
        let y = -(y as f32 / 512. - 1.);
        let dir = Pnt3(x, y, 1.) - origin;
        let intersect_sphere = sphere.ray_intersect(origin, dir);
        let intersect_plane = plane.ray_intersect(origin, dir);
        if intersect_sphere > 0. && (intersect_plane < 0. ||
                                     intersect_plane > intersect_sphere) {
            *pixel = image::Rgb([192u8, 64u8, 64u8]);
        } else if intersect_plane > 0. {
            *pixel = image::Rgb([64u8, 64u8, 192u8]);
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
