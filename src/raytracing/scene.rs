use math::Vector3;
use raytracing::Ray;
use shapes::Sphere;
use std::f64;

pub struct Scene {
    pub spheres: Vec<Sphere>,
    pub light: Sphere,
}

impl Scene {
    pub fn new() -> Scene {
        let scene = Scene {
            spheres: Vec::new(),
            light: Sphere::new_default_color(Vector3::new(0.0, -4.0, 5.0), 1.0),
        };
        return scene;
    }

    pub fn add_sphere(&mut self, sphere: Sphere) {
        self.spheres.push(sphere);
    }

    pub fn trace_scene(&self, ray: &Ray) -> (Option<&Sphere>, f64) {
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

    pub fn compute_color(&self, intersection_point: Vector3, hit_sphere: &Sphere) -> Vector3 {
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
