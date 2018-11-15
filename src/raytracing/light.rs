use math::Vector3;

pub struct Light {
    pub intensity: f64,
    pub origin: Vector3,
}

impl Light {
    pub fn new(intensity: f64, origin: Vector3) -> Light {
        Light {
            intensity,
            origin
        }
    }
}