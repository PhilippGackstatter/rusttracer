extern crate rusttracer;

use rusttracer::ppm;
use std::clone::Clone;
use std::env;
use std::f64;
use std::io::{self, Write};
use std::ops::{Add, Div, Mul, Rem, Sub};

const RANGE_X: i32 = 512;
const RANGE_Y: i32 = 512;

const RANGE_STRETCH_X: f64 = RANGE_X as f64 / 10.0;
const RANGE_STRETCH_Y: f64 = RANGE_Y as f64 / 10.0;

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

#[derive(Debug)]
struct Vector3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Vector3 {
    fn new(x: f64, y: f64, z: f64) -> Vector3 {
        Vector3 { x, y, z }
    }

    fn zero() -> Vector3 {
        Vector3::new(0.0, 0.0, 0.0)
    }

    fn red() -> Vector3 {
        Vector3::new(255.0, 0.0, 0.0)
    }

    fn green() -> Vector3 {
        Vector3::new(0.0, 255.0, 0.0)
    }

    fn normalize(&self) -> Vector3 {
        let len = (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt();
        Vector3 {
            x: self.x / len,
            y: self.y / len,
            z: self.z / len,
        }
    }

    fn len(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }

    fn angle(&self, v: Vector3) -> f64 {
        let angle = (self % &v).acos() / self.len() * v.len();
        angle * 180.0 / std::f64::consts::PI
    }
}

impl Clone for Vector3 {
    fn clone(&self) -> Vector3 {
        Vector3 {
            x: self.x,
            y: self.y,
            z: self.z,
        }
    }
}

impl<'a, 'b> Rem<&'b Vector3> for &'a Vector3 {
    type Output = f64;

    fn rem(self, vec: &'b Vector3) -> f64 {
        self.x * vec.x + self.y + vec.y + self.z + vec.z
    }
}

impl<'a, 'b> Sub<&'b Vector3> for &'a Vector3 {
    type Output = Vector3;

    fn sub(self, vec: &'b Vector3) -> Vector3 {
        Vector3 {
            x: self.x - vec.x,
            y: self.y - vec.y,
            z: self.z - vec.z,
        }
    }
}

impl<'a, 'b> Add<&'b Vector3> for &'a Vector3 {
    type Output = Vector3;

    fn add(self, vec: &'b Vector3) -> Vector3 {
        Vector3 {
            x: self.x + vec.x,
            y: self.y + vec.y,
            z: self.z + vec.z,
        }
    }
}

impl<'a, 'b> Mul<&'b Vector3> for &'a Vector3 {
    type Output = Vector3;

    fn mul(self, vec: &'b Vector3) -> Vector3 {
        Vector3 {
            x: self.x * vec.x,
            y: self.y * vec.y,
            z: self.z * vec.z,
        }
    }
}

impl<'a> Mul<f64> for &'a Vector3 {
    type Output = Vector3;

    fn mul(self, t: f64) -> Vector3 {
        Vector3 {
            x: self.x * t,
            y: self.y * t,
            z: self.z * t,
        }
    }
}

impl<'a> Div<f64> for &'a Vector3 {
    type Output = Vector3;

    fn div(self, di: f64) -> Vector3 {
        Vector3 {
            x: self.x / di,
            y: self.y / di,
            z: self.z / di,
        }
    }
}

#[derive(Debug)]
struct Ray {
    origin: Vector3,
    direction: Vector3,
}

impl Ray {
    fn new(origin: Vector3, direction: Vector3) -> Ray {
        Ray { origin, direction }
    }

    fn get_coordinates(&self, t: f64) -> Vector3 {
        &self.origin + &(&self.direction * t)
    }
}

#[derive(Debug)]
struct Sphere {
    origin: Vector3,
    radius: f64,
    color: Vector3,
}

impl Sphere {
    fn new(origin: Vector3, radius: f64, color: Vector3) -> Sphere {
        Sphere {
            origin,
            radius,
            color,
        }
    }

    fn new_default_color(origin: Vector3, radius: f64) -> Sphere {
        Sphere {
            origin,
            radius,
            color: Vector3::new(255.0, 255.0, 255.0),
        }
    }

    fn get_normal(&self, p: &Vector3) -> Vector3 {
        (p - &self.origin).normalize()
    }

    fn intersect(&self, ray: &Ray) -> Option<f64> {
        let x1 = ray.origin.x;
        let x2 = ray.direction.x + ray.origin.x;
        let x3 = self.origin.x;

        let y1 = ray.origin.y;
        let y2 = ray.direction.y + ray.origin.y;
        let y3 = self.origin.y;

        let z1 = ray.origin.z;
        let z2 = ray.direction.z + ray.origin.z;
        let z3 = self.origin.z;

        let a = (x2 - x1).powi(2) + (y2 - y1).powi(2) + (z2 - z1).powi(2);
        let b = 2.0 * ((x2 - x1) * (x1 - x3) + (y2 - y1) * (y1 - y3) + (z2 - z1) * (z1 - z3));
        let c = x3.powi(2) + y3.powi(2) + z3.powi(2) + x1.powi(2) + y1.powi(2) + z1.powi(2)
            - 2.0 * (x3 * x1 + y3 * y1 + z3 * z1)
            - self.radius.powi(2);

        let discr = b.powi(2) - 4.0 * a * c;

        // TODO Implement with match
        // That requires implementing compare for f64
        if discr < 0.0 {
            // no intersection
            None
        } else if discr > 0.0 {
            let t_plus = (-b + discr.sqrt()) / (2.0 * a);
            let t_minus = (-b - discr.sqrt()) / (2.0 * a);

            // TODO Find the proper way to init t_result
            // let mut t_result = f64::MAX;

            // Return the one thats closer to the rays origin, which is the smaller t
            // TODO Pick the smaller one if both are positive, otherwise the positive one
            // Negative indicates that sth behind the camera was hit
            let t_result = if t_plus < t_minus { t_plus } else { t_minus };

            if t_result < 0.00001 {
                return None;
            }

            Some(t_result)
        } else {
            Some(-b / (2.0 * a))
        }
    }
}

struct Scene {
    spheres: Vec<Sphere>,
    light: Sphere,
}

impl Scene {
    fn new() -> Scene {
        let scene = Scene {
            spheres: Vec::new(),
            light: Sphere::new_default_color(Vector3::new(-5.0, 4.0, 2.5), 1.0),
        };
        return scene;
    }

    fn add_sphere(&mut self, sphere: Sphere) {
        self.spheres.push(sphere);
    }

    fn trace_scene(&self, ray: &Ray) -> (Option<&Sphere>, f64) {
        // TODO Is it possible to write this differently?
        let mut t_result = f64::MAX;
        let mut hit_sphere: Option<&Sphere> = None;

        for sphere in &self.spheres {
            let res = sphere.intersect(ray);

            match res {
                Some(result) => {
                    if result < t_result {
                        t_result = result;
                        hit_sphere = Some(sphere);
                    }
                }
                None => (),
            }
        }

        (hit_sphere, t_result)
    }

    fn compute_color(
        &self,
        intersection_point: Vector3,
        hit_sphere: &Sphere,
    ) -> Vector3 {
        let point_to_light = &self.light.origin - &intersection_point;
        let shadow_ray = Ray::new(intersection_point.clone(), point_to_light.clone());

        // Check where the light sphere intersects with the shadow ray, sent from the
        // hit_spheres surface. This is t_light
        if let Some(t_light) = self.light.intersect(&shadow_ray) {
            // Was t_res
            let (_, t_scene) = self.trace_scene(&shadow_ray);

            if t_scene < t_light {
                // We hit something in the scene before we hit the light
                return Vector3::zero();
            } else {
                // Otherwise we have direct illumination from the light source
                let normal = hit_sphere.get_normal(&intersection_point);
                let angle = normal.angle(point_to_light.normalize());

                if angle > 90.0 {
                    return Vector3::zero();
                } else {
                    // Map angle to a value between [0, 1] and invert it
                    let scalar = 1.0 - angle / 90.0;
                    return &hit_sphere.color * scalar;
                }
            }
        // In this case we should always hit the light, since we have specially constructed a
        // ray that goes from the hit_sphere to the light sphere
        } else {
            Vector3::zero()
        }
    }
}

fn raytrace() -> ppm::PPM {
    let mut scene = Scene::new();
    scene.add_sphere(Sphere::new(
        Vector3::new(0.0, 0.0, 5.0),
        1.5,
        Vector3::red(),
    ));
    scene.add_sphere(Sphere::new(
        Vector3::new(2.0, 4.0, 5.0),
        2.0,
        Vector3::new(255.0, 0.0, 255.0),
    ));

    scene.add_sphere(Sphere::new(
        Vector3::new(2.0, -4.0, 5.0),
        1.5,
        Vector3::new(0.0, 255.0, 255.0),
    ));

    scene.add_sphere(Sphere::new(
        Vector3::new(4.0, 0.0, 5.0),
        1.0,
        Vector3::new(255.0, 255.0, 0.0),
    ));

    scene.add_sphere(Sphere::new(
        Vector3::new(-3.5, 0.0, 5.0),
        0.5,
        Vector3::green(),
    ));

    let mut ppm_img = ppm::PPM::new(RANGE_X as u32, RANGE_Y as u32);

    for i in -RANGE_X..RANGE_X {
        let x = i as f64 / RANGE_STRETCH_X;
        for j in -RANGE_Y..RANGE_Y {
            let y = j as f64 / RANGE_STRETCH_Y;

            let mut color = Vector3::new(127.0, 127.0, 127.0);
            let ray = Ray::new(Vector3::new(x, y, 0.0), Vector3::new(0.0, 0.0, 1.0));
            let (hit_sphere, t) = scene.trace_scene(&ray);

            match hit_sphere {
                Some(sph) => {
                    let intersection_point = ray.get_coordinates(t);
                    color = scene.compute_color(intersection_point, &sph);
                }
                None => (),
            }
            ppm_img.set_pixel(
                (i + RANGE_X) as u32,
                (j + RANGE_Y) as u32,
                ppm::RGB::new(color.x as u8, color.y as u8, color.z as u8),
            );
        }
    }
    ppm_img
}


