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
        // From http://ambrsoft.com/TrigoCalc/Sphere/SpherLineIntersection_.htm
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

        if discr < 0.0 {
            // no intersection
            None
        } else if discr > 0.0 {
            let t_plus = (-b + discr.sqrt()) / (2.0 * a);
            let t_minus = (-b - discr.sqrt()) / (2.0 * a);

            // Return the one thats closer to the rays origin, which is the smaller t
            let t_result = if t_plus < t_minus { t_plus } else { t_minus };

            if t_result < 0.00001 {
                return None;
            }

            Some(t_result)
        } else {
            let t_result = -b / (2.0 * a);
            if t_result < 0.00001 {
                return None;
            } else {
                return Some(t_result);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts;

    #[test]
    fn test_sphere_intersection() {
        let sp = Sphere::new_default_color(Vector3::new(0.0, 0.0, 3.0), 2.0);
        let ray = Ray::new(Vector3::zero(), Vector3::new(0.0, 0.0, 1.0));
        if let Some(t_result) = sp.intersect(&ray) {
            assert_eq!(t_result, 1.0);
        } else {
            panic!();
        }
    }

    #[test]
    fn test_sphere_intersection_2() {
        // Rotate the point 1,0 which is on the circle towards 3,2 by 45 degrees
        // Rotating a point s,t by an angle to u,v:
        // u = s*cos(angle) + t*sin(angle)
        // v = -s*sin(angle) + t*cos(angle)
        // The intersection should then be at height 2 * sin(pi/4) = 1.41...
        // The x value has the same value except shifted by 3
        let sp = Sphere::new_default_color(Vector3::new(0.0, 0.0, 3.0), 2.0);
        let ray = Ray::new(
            Vector3::new(0.0, 2.0 * (consts::PI / 4.0).sin(), 0.0),
            Vector3::new(0.0, 0.0, 1.0),
        );
        if let Some(t_result) = sp.intersect(&ray) {
            assert!((t_result - (3.0 - 2.0 * (consts::PI / 4.0).sin()).abs()) < 1e-14);
        } else {
            panic!();
        }
    }

    #[test]
    fn test_get_normal() {
        let sp = Sphere::new_default_color(Vector3::new(0.0, 0.0, 2.0), 2.0);
        let normal = sp.get_normal(&Vector3::new(0.0, 2.0, 2.0));
        assert_eq!(normal, Vector3::new(0.0, 1.0, 0.0));
    }

    #[test]
    fn test_sphere_intersection_returns_none_on_miss() {
        let sphere = Sphere::new(Vector3::new(0.0, 0.0, 3.0), 1.0, Vector3::red());
        let ray = Ray::new(Vector3::new(0.0, 0.0, 5.0), Vector3::new(0.0, 0.0, 1.0));
        let result = sphere.intersect(&ray);
        assert_eq!(result, None);
    }

    #[test]
    fn test_sphere_intersection_returns_none_on_miss_2() {
        // This ray starts at the top of the sphere
        let sphere = Sphere::new(Vector3::new(0.0, 0.0, 3.0), 1.0, Vector3::red());
        let ray = Ray::new(Vector3::new(0.0, 1.0, 3.0), Vector3::new(0.0, 0.0, 1.0));
        let result = sphere.intersect(&ray);
        assert_eq!(result, None);
    }

    #[test]
    fn test_sphere_intersection_returns_none_on_miss_3() {
        // This ray starts above the sphere
        let sphere = Sphere::new(Vector3::new(0.0, 0.0, 3.0), 1.0, Vector3::red());
        let ray = Ray::new(Vector3::new(0.0, 2.0, 3.0), Vector3::new(0.0, 0.0, 1.0));
        let result = sphere.intersect(&ray);
        assert_eq!(result, None);
    }
}
