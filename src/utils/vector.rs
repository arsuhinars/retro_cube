use core::ops;

use super::approximately;

#[derive(Debug, Clone, Copy, Default)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

pub const ZERO_VECTOR: Vector3 = Vector3 { x: 0.0, y: 0.0, z: 0.0 };
pub const ONE_VECTOR: Vector3 = Vector3 { x: 1.0, y: 1.0, z: 1.0 };
pub const UP_VECTOR: Vector3 = Vector3 { x: 0.0, y: 1.0, z: 0.0 };
pub const RIGHT_VECTOR: Vector3 = Vector3 { x: 1.0, y: 0.0, z: 0.0 };
pub const FORWARD_VECTOR: Vector3 = Vector3 { x: 0.0, y: 0.0, z: 1.0 };

impl Vector3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vector3 {
        Vector3 { x, y, z }
    }

    #[inline]
    pub fn dot(a: &Vector3, b: &Vector3) -> f32 {
        a.x * b.x + a.y * b.y + a.z * b.z
    }

    #[inline]
    pub fn cross(a: &Vector3, b: &Vector3) -> Vector3 {
        Vector3 {
            x: a.y * b.z - a.z * b.y,
            y: a.z * b.x - a.x * b.z,
            z: a.x * b.y - a.y * b.x
        }
    }

    #[inline]
    pub fn sqr_length(&self) -> f32 {
        self.x.powi(2) + self.y.powi(2) + self.z.powi(2)
    }

    #[inline]
    pub fn length(&self) -> f32 {
        self.sqr_length().sqrt()
    }

    #[inline]
    pub fn normalized(&self) -> Vector3 {
        let l = self.length();

        Vector3 {
            x: self.x / l, y: self.y / l, z: self.z / l
        }
    }

    #[inline]
    pub fn angle(a: &Vector3, b: &Vector3) -> f32 {
        (Vector3::dot(a, b) / (a.length() * b.length())).acos()
    }

    #[inline]
    pub fn approximately(&self, v: &Vector3) -> bool {
        approximately(self.x, v.x) &&
        approximately(self.y, v.y) &&
        approximately(self.z, v.z)
    }

    #[inline]
    pub fn abs(&self) -> Vector3 {
        Vector3 {
            x: self.x.abs(), y: self.y.abs(), z: self.z.abs()
        }
    }

    #[inline]
    pub fn to_degrees(&self) -> Vector3 {
        Vector3 {
            x: self.x.to_degrees(),
            y: self.y.to_degrees(),
            z: self.z.to_degrees()
        }
    }

    #[inline]
    pub fn to_radians(&self) -> Vector3 {
        Vector3 {
            x: self.x.to_radians(),
            y: self.y.to_radians(),
            z: self.z.to_radians()
        }
    }
}

impl ops::Add<Vector3> for Vector3 {
    type Output = Vector3;

    fn add(self, rhs: Vector3) -> Self::Output {
        Vector3 { x: self.x + rhs.x, y: self.y + rhs.y, z: self.z + rhs.z }
    }
}

impl ops::AddAssign<Vector3> for Vector3 {
    fn add_assign(&mut self, rhs: Vector3) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl ops::Add<&Vector3> for Vector3 {
    type Output = Vector3;

    fn add(self, rhs: &Vector3) -> Self::Output {
        Vector3 { x: self.x + rhs.x, y: self.y + rhs.y, z: self.z + rhs.z }
    }
}

impl ops::AddAssign<&Vector3> for Vector3 {
    fn add_assign(&mut self, rhs: &Vector3) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl ops::Sub<Vector3> for Vector3 {
    type Output = Vector3;

    fn sub(self, rhs: Vector3) -> Self::Output {
        Vector3 { x: self.x - rhs.x, y: self.y - rhs.y, z: self.z - rhs.z }
    }
}

impl ops::SubAssign<Vector3> for Vector3 {
    fn sub_assign(&mut self, rhs: Vector3) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl ops::Sub<&Vector3> for Vector3 {
    type Output = Vector3;

    fn sub(self, rhs: &Vector3) -> Self::Output {
        Vector3 { x: self.x - rhs.x, y: self.y - rhs.y, z: self.z - rhs.z }
    }
}

impl ops::SubAssign<&Vector3> for Vector3 {
    fn sub_assign(&mut self, rhs: &Vector3) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl ops::Mul<Vector3> for Vector3 {
    type Output = Vector3;

    fn mul(self, rhs: Vector3) -> Self::Output {
        Vector3 { x: self.x * rhs.x, y: self.y * rhs.y, z: self.z * rhs.z }
    }
}

impl ops::Mul<&Vector3> for Vector3 {
    type Output = Vector3;

    fn mul(self, rhs: &Vector3) -> Self::Output {
        Vector3 { x: self.x * rhs.x, y: self.y * rhs.y, z: self.z * rhs.z }
    }
}

impl ops::Mul<f32> for Vector3 {
    type Output = Vector3;

    fn mul(self, rhs: f32) -> Self::Output {
        Vector3 { x: self.x * rhs, y: self.y * rhs, z: self.z * rhs }
    }
}

impl ops::Mul<Vector3> for f32 {
    type Output = Vector3;

    fn mul(self, rhs: Vector3) -> Self::Output {
        Vector3 { x: self * rhs.x, y: self * rhs.y, z: self * rhs.z }
    }
}

impl ops::Neg for Vector3 {
    type Output = Vector3;

    fn neg(self) -> Self::Output {
        Vector3 { x: -self.x, y: -self.y, z: -self.z }
    }
}

impl From<mint::Vector3<f32>> for Vector3 {
    fn from(value: mint::Vector3<f32>) -> Self {
        Vector3 {
            x: value.x,
            y: value.y,
            z: value.z
        }
    }
}

impl From<Vector3> for mint::Vector3<f32> {
    fn from(value: Vector3) -> Self {
        mint::Vector3 { x: value.x, y: value.y, z: value.z }
    }
}

impl From<[f32; 3]> for Vector3 {
    fn from(value: [f32; 3]) -> Self {
        Vector3 {
            x: value[0], y: value[1], z: value[2]
        }
    }
}

impl From<Vector3> for [f32; 3] {
    fn from(value: Vector3) -> Self {
        [value.x, value.y, value.z]
    }
}
