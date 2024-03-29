use math::Vector3;
use raytracing::Light;
use raytracing::Ray;
use shapes::Sphere;
use std::f64;

pub struct Scene {
    pub spheres: Vec<Sphere>,
    pub lights: Vec<Light>,
    pub ambient_light: f64,
}

impl Scene {
    pub fn new(lights: Vec<Light>, ambient_light: f64) -> Scene {
        Scene {
            spheres: Vec::new(),
            lights,
            ambient_light,
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

    pub fn compute_color(
        &self,
        ray: &Ray,
        intersection_point: Vector3,
        hit_sphere: &Sphere,
    ) -> Vector3 {
        // This is the lambertian coefficient for *this* object.
        // This should be supplied by every object's material.
        let lambertian_coefficient = 1.7;
        // This determines the size of the specular highlight
        let shininess_factor = 25;
        // The base color is the color of the object scaled by the ambient light intensity
        let mut color = &Vector3::zero() + &(&hit_sphere.color * self.ambient_light);

        for light in self.lights.iter() {
            let point_to_light = &light.origin - &intersection_point;
            // Get the distance t_light to the light
            let t_light = point_to_light.len();

            // We have to hit the light, therefore have to have a result,
            // otherwise the calculation setup was wrong
            // let t_light = light.intersect(&shadow_ray).unwrap();

            let shadow_ray = Ray::new(intersection_point.clone(), point_to_light.clone());
            let (hit_object, t_scene) = self.trace_scene(&shadow_ray);

            // Only if we didnt hit anything or if the light is closer than the object we hit
            // -- meaning, there is no object between this point and the light -- do we calculate shading
            if hit_object.is_none() || t_light < t_scene {
                // We have illumination from the light source
                let normal = hit_sphere.get_normal(&intersection_point);

                // Lambert Shading
                let lambert_contribution =
                    self.lambert_shading(&normal, &point_to_light, lambertian_coefficient)
                        * light.intensity;
                color = &color + &(&color * lambert_contribution);

                // Specular Shading
                let specular_contribution =
                    self.specular_shading(&ray, normal, point_to_light, shininess_factor);
                color = &color + &(&color * specular_contribution);
            }

            // Don't allow values larger than 255
            color.x = color.x.min(255.0);
            color.y = color.y.min(255.0);
            color.z = color.z.min(255.0);
        }
        color
    }

    fn lambert_shading(
        &self,
        normal: &Vector3,
        to_light: &Vector3,
        lambertian_coefficient: f64,
    ) -> f64 {
        let dot_prod = normal % &to_light.normalize();
        // Negative dot products mean the angle was larger than 90, so we ignore
        // the contribution in that case
        (dot_prod * lambertian_coefficient).max(0.0)
    }

    fn specular_shading(
        &self,
        ray: &Ray,
        normal: Vector3,
        to_light: Vector3,
        shininess_factor: i32,
    ) -> f64 {
        // We want to get the cosine of the angle between the vector pointing towards the
        // camera and the reflected vector of the light
        // If they are almost the same, the value (dot product) will be close to 1, meaning the reflection
        // of the light is pointing directly towards the camera. Here is where we would expect
        // a specular highlight.
        // If the angle is larger, the value will be closer to 0. This is perfect.
        // We can simply take this value and multiply it with the color, and add it to the existing value.
        let inverse_view_direction = ray.direction.inverse();
        let incoming_light_direction = to_light.inverse();
        let reflected_light_ray = incoming_light_direction.reflect(&normal);
        // We normalize both vectors because we are interested in the angle between them
        // This will result in values between -1 and 1.
        let dot_prod = &inverse_view_direction.normalize() % &(reflected_light_ray).normalize();
        let specular_contribution = dot_prod.powi(shininess_factor);
        // Only return values greater than 0
        return specular_contribution.max(0.0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_color() {
        // The light is above the sphere and doesnt intersect with it
        let lights = vec![Light::new(1.2, Vector3::new(0.0, 7.0, 3.0))];
        let mut scene = Scene::new(lights, 0.1);

        let sphere = Sphere::new(Vector3::new(0.0, 0.0, 3.0), 1.0, Vector3::red());
        let ray = Ray::new(Vector3::new(0.0, 0.0, 0.0), Vector3::new(0.0, 0.0, 0.0));
        scene.add_sphere(sphere);

        let color = scene.compute_color(&ray, Vector3::new(0.0, 1.0, 3.0), &scene.spheres[0]);

        assert_eq!(
            color,
            &(&Vector3::red() * scene.ambient_light)
                + &(&Vector3::red() * (scene.ambient_light * 2.8))
        );
    }

}
