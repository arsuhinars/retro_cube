use std::{ops, convert};
use super::vector::{Vector3, RIGHT_VECTOR};

#[derive(Default, Clone, Copy)]
pub struct Matrix3([[f32; 3]; 3]);

#[derive(Default, Clone, Copy)]
pub struct Matrix4([[f32; 4]; 4]);

pub fn translate_matrix(translate: &Vector3) -> Matrix4 {
    Matrix4([
        [1.0, 0.0, 0.0, translate.x],
        [0.0, 1.0, 0.0, translate.y],
        [0.0, 0.0, 1.0, translate.z],
        [0.0, 0.0, 0.0, 1.0]
    ])
}

pub fn look_at_rotate_matrix(direction: &Vector3) -> Matrix3 {
    let forward = direction.normalized();
    let up = Vector3::cross(&RIGHT_VECTOR, &forward).normalized();
    let right = Vector3::cross(&forward, &up);

    return Matrix3([
        [right.x, up.x, forward.x],
        [right.y, up.y, forward.y],
        [right.z, up.z, forward.z]
    ])
}

pub fn inverse_look_at_rotate_matrix(direction: &Vector3) -> Matrix3 {
    let forward = direction.normalized();
    let up = Vector3::cross(&RIGHT_VECTOR, &forward).normalized();
    let right = Vector3::cross(&forward, &up);

    return Matrix3([
        [right.x, right.y, right.z],
        [up.x, up.y, up.z],
        [forward.x, forward.y, forward.z]
    ])
}

pub fn euler_rotation_matrix(rotation: &Vector3) -> Matrix3 {
    let (cos_x, cos_y, cos_z) = (rotation.x.cos(), rotation.y.cos(), rotation.z.cos());
    let (sin_x, sin_y, sin_z) = (rotation.x.sin(), rotation.y.sin(), rotation.z.sin());

    let mut m = Matrix3::default();
    m[0][0] = cos_z * cos_y;
    m[0][1] = cos_z * sin_y * sin_x - sin_z * cos_x;
    m[0][2] = cos_z * sin_y * cos_x + sin_z * sin_x;
    m[1][0] = sin_z * cos_y;
    m[1][1] = sin_z * sin_y * sin_x + cos_z * cos_x;
    m[1][2] = sin_z * sin_y * cos_x - cos_z * sin_x;
    m[2][0] = -sin_y;
    m[2][1] = cos_y * sin_x;
    m[2][2] = cos_y * cos_x;

    return m;
}

pub fn inverse_euler_rotation_matrix(rotation: &Vector3) -> Matrix3 {
    let (cos_x, cos_y, cos_z) = (rotation.x.cos(), rotation.y.cos(), rotation.z.cos());
    let (sin_x, sin_y, sin_z) = (rotation.x.sin(), rotation.y.sin(), rotation.z.sin());

    let mut m: [[f32; 3]; 3] = [[0.0; 3]; 3];
    m[0][0] = cos_z * cos_y;
    m[0][1] = sin_z * cos_y;
    m[0][2] = -sin_y;
    m[1][0] = cos_z * sin_y * sin_x - sin_z * cos_x;
    m[1][1] = sin_z * sin_y * sin_x + cos_z * cos_x;
    m[1][2] = cos_y * sin_x;
    m[2][0] = cos_z * sin_y * cos_x + sin_z * sin_x;
    m[2][1] = sin_z * sin_y * cos_x - cos_z * sin_x;
    m[2][2] = cos_y * cos_x;

    return Matrix3(m);
}

pub fn scale_matrix(scale: &Vector3) -> Matrix3 {
    Matrix3([
        [scale.x, 0.0, 0.0],
        [0.0, scale.y, 0.0],
        [0.0, 0.0, scale.z]
    ])
}

impl ops::Index<usize> for Matrix3 {
    type Output = [f32; 3];

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl ops::IndexMut<usize> for Matrix3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl ops::Mul<Matrix3> for Matrix3 {
    type Output = Matrix3;

    fn mul(self, rhs: Matrix3) -> Self::Output {
        let mut m = Matrix3::default();

        for i in 0..3 {
            for j in 0..3 {
                let mut v = 0.0;
                for k in 0..3 {
                    v += self[i][k] * rhs[k][j];
                }
                m[i][j] = v;
            }
        }

        return m;
    }
}

impl ops::Mul<Vector3> for Matrix3 {
    type Output = Vector3;

    fn mul(self, rhs: Vector3) -> Self::Output {
        Vector3 {
            x: rhs.x * self[0][0] + rhs.y * self[0][1] + rhs.z * self[0][2],
            y: rhs.x * self[1][0] + rhs.y * self[1][1] + rhs.z * self[1][2],
            z: rhs.x * self[2][0] + rhs.y * self[2][1] + rhs.z * self[2][2]
        }
    }
}

impl convert::From<Matrix4> for Matrix3 {
    fn from(value: Matrix4) -> Self {
        let mut m = Matrix3::default();

        for i in 0..3 {
            for j in 0..3 {
                m[i][j] = value[i][j];
            }
        }

        return m;
    }
}

impl ops::Index<usize> for Matrix4 {
    type Output = [f32; 4];

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl ops::IndexMut<usize> for Matrix4 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl ops::Mul<Matrix4> for Matrix4 {
    type Output = Matrix4;

    fn mul(self, rhs: Matrix4) -> Self::Output {
        let mut m = Matrix4::default();

        for i in 0..4 {
            for j in 0..4 {
                let mut v = 0.0;
                for k in 0..4 {
                    v += self[i][k] * rhs[k][j];
                }
                m[i][j] = v;
            }
        }

        return m;
    }
}

impl ops::Mul<Vector3> for Matrix4 {
    type Output = Vector3;

    fn mul(self, rhs: Vector3) -> Self::Output {
        Vector3 {
            x: rhs.x * self[0][0] + rhs.y * self[0][1] + rhs.z * self[0][2] + self[0][3],
            y: rhs.x * self[1][0] + rhs.y * self[1][1] + rhs.z * self[1][2] + self[1][3],
            z: rhs.x * self[2][0] + rhs.y * self[2][1] + rhs.z * self[2][2] + self[2][3]
        }
    }
}

impl convert::From<Matrix3> for Matrix4 {
    fn from(value: Matrix3) -> Self {
        let mut m = Matrix4::default();

        for i in 0..3 {
            for j in 0..3 {
                m[i][j] = value[i][j];
            }
        }

        m[3][3] = 1.0;

        return m;
    }
}
