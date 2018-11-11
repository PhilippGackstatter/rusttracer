extern crate rusttracer;

use rusttracer::math::Vector3;
use rusttracer::ppm;
use rusttracer::raytracing::{Camera, Scene, Sphere};
use std::env;
use std::f64;
use std::io::{self, Write};

const WIDTH: u16 = 512;
const HEIGHT: u16 = 512;

fn main() {
    let ppm = raytrace();

    if let Some(arg) = env::args().nth(1) {
        ppm.write_file(&arg).expect("Could not write file");
    } else {
        // Call like cargo run | display
        if let Err(e) = io::stdout().write(&ppm.get_bytes()) {
            println!("Could not write to stdout {}", e);
        }
    }
}

fn add_spheres(scene: &mut Scene) {
    scene.add_sphere(Sphere::new(
        Vector3::new(0.0, 0.0, 5.0),
        1.5,
        Vector3::red(),
    ));
    // scene.add_sphere(Sphere::new(
    //     Vector3::new(-2.0, -2.0, 5.0),
    //     1.0,
    //     Vector3::purple()
    // ));

    // scene.add_sphere(Sphere::new(
    //     Vector3::new(2.0, 2.0, 5.0),
    //     1.5,
    //     Vector3::orange()
    // ));

    // scene.add_sphere(Sphere::new(
    //     Vector3::new(-3.5, 2.5, 8.0),
    //     1.5,
    //     Vector3::green(),
    // ));
}

fn raytrace() -> ppm::PPM {
    let mut scene = Scene::new();
    add_spheres(&mut scene);
    let camera = Camera::new_at_zero(WIDTH as f64, HEIGHT as f64, 90.0);
    let mut ppm_img = ppm::PPM::new(WIDTH as u32, HEIGHT as u32);

    for x in 0..WIDTH {
        for y in 0..HEIGHT {
            let rgb = camera.get_pixel_color(&scene, x, y);
            ppm_img.set_pixel(x as u32, y as u32, rgb);
        }
    }
    ppm_img
}
