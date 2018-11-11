use math::Vector3;
use raytracing::{Ray, Scene};
use ppm;

pub struct Camera {
    origin: Vector3,
    width: f64,
    height: f64,
    fov: f64,
}

impl Camera {
    pub fn new_at_zero(width: f64, height: f64, fov: f64) -> Camera {
        // Constructs a new Camera with the given width and height and a fov given in degrees
        Camera {
            origin: Vector3::zero(),
            width,
            height,
            fov: (fov / 2.0).to_radians(),
        }
    }

    pub fn new(position: Vector3, width: f64, height: f64, fov: f64) -> Camera {
        // Constructs a new Camera with the given position, width, height and a fov given in degrees
        Camera {
            origin: position,
            width,
            height,
            fov: (fov / 2.0).to_radians(),
        }
    }

    pub fn map_pixel_to_plane(&self, x: u16, y: u16) -> Vector3 {
        // For a given pixel pair, returns the position on the perspective camera plane
        let x_coord = (2.0 * x as f64 - self.width) / self.width * (self.fov).tan();
        let y_coord = (2.0 * y as f64 - self.height) / self.height
            * (self.height / self.width * self.fov).tan();
        Vector3::new(x_coord, y_coord, 1.0)
    }

    pub fn get_camera_ray(&self, x: u16, y: u16) -> Ray {
        // Gets the camera ray going from the camera origin through pixel x, y
        // on the camera plane
        let plane_position = self.map_pixel_to_plane(x, y);
        Ray::new(self.origin.clone(), plane_position)
    }

    pub fn get_pixel_color(&self, scene: &Scene, x: u16, y: u16) -> ppm::RGB {
        let mut color = Vector3::new(127.0, 127.0, 127.0);
        let ray = self.get_camera_ray(x, y);
        let (hit_sphere, t) = scene.trace_scene(&ray);

        match hit_sphere {
            Some(sph) => {
                let intersection_point = ray.get_coordinates(t);
                color = scene.compute_color(intersection_point, &sph);
            }
            None => (),
        }
        ppm::RGB::new(color.x as u8, color.y as u8, color.z as u8)
    }
 }

#[test]
fn test_camera_returns_corners() {
    // Passing 0, 0 should return -1, -1 the lower left corner of the plane
    // Whereas passing width, height, should return 1, 1 the upper right corner
    let width = 16.0;
    let height = 16.0;
    let camera = Camera::new_at_zero(width, height, 90.0);
    let vec2 = camera.map_pixel_to_plane(0, 0);

    // For a quadratic camera and a standard fov of 90
    // these tests should hold true because
    assert!(((90.0_f64 / 2.0_f64).to_radians().tan() - 1.0) < 1e-14);
    // which just says that the tan of 45 as radians is 1.
    // We expect x and y to be -1.0
    assert!((vec2.x + 1.0) < 1e-14);
    assert!((vec2.y + 1.0) < 1e-14);

    let vec2 = camera.map_pixel_to_plane(width as u16, height as u16);

    // We expect x and y to be 1.0
    assert!((vec2.x - 1.0) < 1e-14);
    assert!((vec2.y - 1.0) < 1e-14);
}
