use math::Vector3;
use raytracing::Ray;
use shapes::Sphere;
use std::f64;

pub struct Scene {
    pub spheres: Vec<Sphere>,
    pub light: Sphere,
    pub ambient_light: f64,
}

impl Scene {
    pub fn new(light: Sphere) -> Scene {
        Scene {
            spheres: Vec::new(),
            light,
            ambient_light: 0.2,
        }
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
        // This is the lambertian coefficient for *this* object.
        // This should be supplied by every object's material.
        let diffuse_coefficient = 2.8;
        // The base color is the color of the object scaled by the ambient light intensity
        let mut color = &Vector3::zero() + &(&hit_sphere.color * self.ambient_light);

        // Check at what distnace t_light the light sphere intersects with the shadow ray,
        // sent from the hit_spheres surface.
        if let Some(t_light) = self.light.intersect(&shadow_ray) {
            let (hit_object, t_scene) = self.trace_scene(&shadow_ray);

            // Only if we didnt hit anything or if the light is closer than the object we hit
            // meaning, there is no object between this point and the light do we calculate shading
            if hit_object.is_none() || t_light > t_scene {
                // Otherwise we have illumination from the light source
                let normal = hit_sphere.get_normal(&intersection_point);
                let mut dot_prod = &normal % &point_to_light.normalize();
                // Negative dot products mean the angle was larger than 90, so we ignore
                // the contribution from this light
                if dot_prod > 0.0 {
                    color = &color + &(&color * (dot_prod * diffuse_coefficient));
                }
            }
        }

        return color;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_color() {
        // The light is above the sphere and doesnt intersect with it
        let mut scene = Scene::new(Sphere::new_default_color(Vector3::new(0.0, 7.0, 3.0), 1.0));
        let sphere = Sphere::new(Vector3::new(0.0, 0.0, 3.0), 1.0, Vector3::red());
        scene.add_sphere(sphere);

        let color = scene.compute_color(Vector3::new(0.0, 1.0, 3.0), &scene.spheres[0]);

        assert_eq!(
            color,
            &(&Vector3::red() * scene.ambient_light)
                + &(&Vector3::red() * (scene.ambient_light * 2.8))
        );
    }

}
