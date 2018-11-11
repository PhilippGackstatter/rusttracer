use math::Vector3;
use raytracing::Ray;

#[derive(Debug)]
pub struct Sphere {
    pub origin: Vector3,
    pub radius: f64,
    pub color: Vector3,
}

impl Sphere {
    pub fn new(origin: Vector3, radius: f64, color: Vector3) -> Sphere {
        Sphere {
            origin,
            radius,
            color,
        }
    }

    pub fn new_default_color(origin: Vector3, radius: f64) -> Sphere {
        Sphere {
            origin,
            radius,
            color: Vector3::new(255.0, 255.0, 255.0),
        }
    }

    pub fn get_normal(&self, p: &Vector3) -> Vector3 {
        (p - &self.origin).normalize()
    }

    pub fn intersect(&self, ray: &Ray) -> Option<f64> {
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