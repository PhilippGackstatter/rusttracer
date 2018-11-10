pub struct Camera {
    width: f64,
    height: f64,
    fov: f64,
}

impl Camera {
    pub fn new(width: f64, height: f64, fov: f64) -> Camera {
        // Constructs a new Camera with the given width and heigth and a fov given in degrees
        Camera {
            width,
            height,
            fov: (fov / 2.0).to_radians(),
        }
    }

    pub fn map_pixel_to_plane(&self, x: u16, y: u16) -> (f64, f64) {
        // For a given pixel pair, returns the position on the perspective camera plane
        let x_coord = (2.0 * x as f64 - self.width) / self.width * (self.fov).tan();
        let y_coord = (2.0 * y as f64 - self.height) / self.height
            * (self.height / self.width * self.fov).tan();;
        (x_coord, y_coord)
    }
}

#[test]
fn test_camera_returns_corners() {
    // Passing 0, 0 should return -1, -1 the lower left corner of the plane
    // Whereas passing width, height, should return 1, 1 the upper right corner
    let width = 16.0;
    let height = 16.0;
    let camera = Camera::new(width, height, 90.0);
    let (x, y) = camera.map_pixel_to_plane(0, 0);

    // For a quadratic camera and a standard fov of 90
    // these tests should hold true because
    assert!(((90.0_f64 / 2.0_f64).to_radians().tan() - 1.0) < 1e-14);
    // which just says that the tan of 45 as radians is 1.
    // We expect x and y to be -1.0
    assert!((x + 1.0) < 1e-14);
    assert!((y + 1.0) < 1e-14);

    let (x, y) = camera.map_pixel_to_plane(width as u16, height as u16);

    // We expect x and y to be 1.0
    assert!((x - 1.0) < 1e-14);
    assert!((y - 1.0) < 1e-14);
}
