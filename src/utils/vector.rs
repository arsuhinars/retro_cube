use core::ops;
use crate::utils::approximately;

#[derive(Debug, Clone, Copy, Default)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

pub const ZERO_VECTOR: Vector3 = Vector3 { x: 0.0, y: 0.0, z: 0.0 };
pub const UP_VECTOR: Vector3 = Vector3 { x: 0.0, y: 1.0, z: 0.0 };
pub const RIGHT_VECTOR: Vector3 = Vector3 { x: 1.0, y: 0.0, z: 0.0 };
pub const FORWARD_VECTOR: Vector3 = Vector3 { x: 0.0, y: 0.0, z: 1.0 };

impl Vector3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vector3 {
        Vector3 { x, y, z }
    }

    pub fn dot(a: &Vector3, b: &Vector3) -> f32 {
        a.x * b.x + a.y * b.y + a.z * b.z
    }

    pub fn cross(a: &Vector3, b: &Vector3) -> Vector3 {
        Vector3 {
            x: a.y * b.z - a.z * b.y,
            y: a.z * b.x - a.x * b.z,
            z: a.x * b.y - a.y * b.x
        }
    }

    pub fn length(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn normalized(&self) -> Vector3 {
        let l = self.length();

        Vector3 {
            x: self.x / l, y: self.y / l, z: self.z / l
        }
    }

    pub fn angle(a: &Vector3, b: &Vector3) -> f32 {
        (Vector3::dot(a, b) / (a.length() * b.length())).acos()
    }

    pub fn approximately(&self, v: &Vector3) -> bool {
        approximately(self.x, v.x) &&
        approximately(self.y, v.y) &&
        approximately(self.z, v.z)
    }

    pub fn look_at_rotate(vec: &Vector3, dir: &Vector3) -> Vector3 {
        let forward = dir.normalized();
        let up = Vector3::cross(&RIGHT_VECTOR, &forward).normalized();
        let right = Vector3::cross(&forward, &up);

        return vec.x * right + vec.y * up + vec.z * forward;
    }

    pub fn inverse_look_at_rotate(vec: &Vector3, dir: &Vector3) -> Vector3 {
        let forward = dir.normalized();
        let up = Vector3::cross(&RIGHT_VECTOR, &forward).normalized();
        let right = Vector3::cross(&forward, &up).normalized();

        Vector3 {
            x: Vector3::dot(vec, &right),
            y: Vector3::dot(vec, &up),
            z: Vector3::dot(vec, &forward)
        }
    }

    pub fn euler_rotate(vec: &Vector3, angles: &Vector3) -> Vector3 {
        let (cos_x, cos_y, cos_z) = (angles.x.cos(), angles.y.cos(), angles.z.cos());
        let (sin_x, sin_y, sin_z) = (angles.x.sin(), angles.y.sin(), angles.z.sin());

        let mut matrix: [[f32; 3]; 3] = [[0.0; 3]; 3];
        matrix[0][0] = cos_z * cos_y;
        matrix[0][1] = cos_z * sin_y * sin_x - sin_z * cos_x;
        matrix[0][2] = cos_z * sin_y * cos_x + sin_z * sin_x;
        matrix[1][0] = sin_z * cos_y;
        matrix[1][1] = sin_z * sin_y * sin_x + cos_z * cos_x;
        matrix[1][2] = sin_z * sin_y * cos_x - cos_z * sin_x;
        matrix[2][0] = -sin_y;
        matrix[2][1] = cos_y * sin_x;
        matrix[2][2] = cos_y * cos_x;

        return (*vec) * matrix;
    }

    pub fn inverse_euler_rotate(vec: &Vector3, angles: &Vector3) -> Vector3 {
        let (cos_x, cos_y, cos_z) = (angles.x.cos(), angles.y.cos(), angles.z.cos());
        let (sin_x, sin_y, sin_z) = (angles.x.sin(), angles.y.sin(), angles.z.sin());

        let mut matrix: [[f32; 3]; 3] = [[0.0; 3]; 3];
        matrix[0][0] = cos_z * cos_y;
        matrix[0][1] = sin_z * cos_y;
        matrix[0][2] = -sin_y;
        matrix[1][0] = cos_z * sin_y * sin_x - sin_z * cos_x;
        matrix[1][1] = sin_z * sin_y * sin_x + cos_z * cos_x;
        matrix[1][2] = cos_y * sin_x;
        matrix[2][0] = cos_z * sin_y * cos_x + sin_z * sin_x;
        matrix[2][1] = sin_z * sin_y * cos_x - cos_z * sin_x;
        matrix[2][2] = cos_y * cos_x;

        return (*vec) * matrix;
    }
}

impl ops::Add<Vector3> for Vector3 {
    type Output = Vector3;

    fn add(self, rhs: Vector3) -> Self::Output {
        Vector3 { x: self.x + rhs.x, y: self.y + rhs.y, z: self.z + rhs.z }
    }
}

impl ops::Add<&Vector3> for Vector3 {
    type Output = Vector3;

    fn add(self, rhs: &Vector3) -> Self::Output {
        Vector3 { x: self.x + rhs.x, y: self.y + rhs.y, z: self.z + rhs.z }
    }
}

impl ops::Sub<Vector3> for Vector3 {
    type Output = Vector3;

    fn sub(self, rhs: Vector3) -> Self::Output {
        Vector3 { x: self.x - rhs.x, y: self.y - rhs.y, z: self.z - rhs.z }
    }
}

impl ops::Sub<&Vector3> for Vector3 {
    type Output = Vector3;

    fn sub(self, rhs: &Vector3) -> Self::Output {
        Vector3 { x: self.x - rhs.x, y: self.y - rhs.y, z: self.z - rhs.z }
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

impl ops::Mul<[[f32; 3]; 3]> for Vector3 {
    type Output = Vector3;

    fn mul(self, m: [[f32; 3]; 3]) -> Self::Output {
        Vector3 {
            x: self.x * m[0][0] + self.y * m[1][0] + self.z * m[2][0],
            y: self.x * m[0][1] + self.y * m[1][1] + self.z * m[2][1],
            z: self.x * m[0][2] + self.y * m[1][2] + self.z * m[2][2],
        }
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
