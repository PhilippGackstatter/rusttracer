use std::clone::Clone;
use std::f64;
use std::ops::{Add, Div, Mul, Rem, Sub};
use std::cmp::PartialEq;

#[derive(Debug)]
pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vector3 {
        Vector3 { x, y, z }
    }

    pub fn zero() -> Vector3 {
        Vector3::new(0.0, 0.0, 0.0)
    }

    pub fn red() -> Vector3 {
        Vector3::new(255.0, 0.0, 0.0)
    }

    pub fn green() -> Vector3 {
        Vector3::new(0.0, 255.0, 0.0)
    }

    pub fn purple() -> Vector3 {
        Vector3::new(255.0, 0.0, 255.0)
    }

    pub fn cyan() -> Vector3 {
        Vector3::new(255.0, 255.0, 0.0)
    }

    pub fn orange() -> Vector3 {
        Vector3::new(255.0, 153.0, 0.0)
    }

    pub fn normalize(&self) -> Vector3 {
        let len = (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt();
        Vector3 {
            x: self.x / len,
            y: self.y / len,
            z: self.z / len,
        }
    }

    pub fn len(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }

    pub fn angle(&self, v: Vector3) -> f64 {
        let dot_product = self % &v;
        let angle = dot_product.acos() / self.len() * v.len();
        angle * 180.0 / f64::consts::PI
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
        self.x * vec.x + self.y * vec.y + self.z * vec.z
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


impl PartialEq for Vector3 {
    fn eq(&self, other: &Vector3) -> bool {
        self.x == other.x &&
        self.y == other.y &&
        self.z == other.z
    }
}

impl Eq for Vector3 {}