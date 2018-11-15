extern crate argparse;
extern crate png;
extern crate rusttracer;

use argparse::{ArgumentParser, Store, StoreTrue};
use rusttracer::math::Vector3;
use rusttracer::util::ppm;
use rusttracer::raytracing::{Camera, Scene};
use rusttracer::shapes::Sphere;
use rusttracer::raytracing::Light;
use rusttracer::util::image_output;
use std::f64;
use std::io::{self, Write};

const WIDTH: u16 = 512;
const HEIGHT: u16 = 512;

fn main() {
    let mut field_of_view = 75.0;
    let mut write_file = "".to_string();
    let mut write_to_stdout = false;
    {
        let mut parser = ArgumentParser::new();
        parser.set_description("A raytracer");
        parser
            .refer(&mut field_of_view)
            .add_option(&["-f", "--fov"], Store, "The field of view.");
        parser.refer(&mut write_file).add_option(
            &["-w", "--write-file"],
            Store,
            "Write output to a file.",
        );
        parser.refer(&mut write_to_stdout).add_option(
            &["-s", "--write-stdout"],
            StoreTrue,
            "Write output to stdout.",
        );

        parser.parse_args_or_exit();
    }

    let ppm = raytrace(field_of_view);

    if write_file != "" {
        image_output::write_png_img(
            &ppm.get_raw_bytes(),
            ppm.get_width(),
            ppm.get_height(),
            write_file,
        );
        // ppm.write_file(&write_file).expect("Could not write file");
    }

    if write_to_stdout {
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
    scene.add_sphere(Sphere::new(
        Vector3::new(-2.5, -2.0, 8.0),
        1.0,
        Vector3::purple(),
    ));

    scene.add_sphere(Sphere::new(
        Vector3::new(2.0, 2.0, 5.0),
        1.0,
        Vector3::orange(),
    ));

    scene.add_sphere(Sphere::new(
        Vector3::new(-3.5, -5.0, 5.0),
        0.8,
        Vector3::green(),
    ));
}

fn raytrace(fov: f64) -> ppm::PPM {
    let lights = vec![
        Light::new(1.2, Vector3::new(0.0, -5.0, 4.0)),
        Light::new(1.9, Vector3::new(-5.0, 0.0, 4.0)),
        Light::new(1.5, Vector3::new(5.0, 0.0, 4.0))
    ];
    let mut scene = Scene::new(lights, 0.1);
    add_spheres(&mut scene);
    let camera = Camera::new(
        Vector3::new(0.0, 0.0, -5.0),
        WIDTH as f64,
        HEIGHT as f64,
        fov,
    );
    let mut ppm_img = ppm::PPM::new(WIDTH as u32, HEIGHT as u32);

    for x in 0..WIDTH {
        for y in 0..HEIGHT {
            let rgb = camera.get_pixel_color(&scene, x, y);
            ppm_img.set_pixel(x as u32, y as u32, rgb);
        }
    }
    ppm_img
}
