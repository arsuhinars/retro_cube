use super::vector::Vector3;
use super::matrix::{Matrix3, Matrix4, translate_matrix, euler_rotation_matrix, inverse_euler_rotation_matrix};

#[derive(Default, Clone, Copy)]
pub struct Transform {
    position: Vector3,
    rotation: Vector3,
    dirty_flag: bool,
    position_matrix: Matrix4,
    inverse_position_matrix: Matrix4,
    direction_matrix: Matrix3,
    inverse_direction_matrix: Matrix3
}

impl Transform {
    pub fn new(position: &Vector3, rotation: &Vector3) -> Transform {
        Transform {
            position: *position,
            rotation: *rotation,
            dirty_flag: true,
            position_matrix: Matrix4::default(),
            inverse_position_matrix: Matrix4::default(),
            direction_matrix: Matrix3::default(),
            inverse_direction_matrix: Matrix3::default()
        }
    }

    pub fn get_position(&self) -> Vector3 { self.position }

    pub fn set_position(&mut self, position: &Vector3) {
        self.position = *position;
        self.dirty_flag = true;
    }

    pub fn get_rotation(&self) -> Vector3 { self.rotation }

    pub fn set_rotation(&mut self, rotation: &Vector3) {
        self.rotation = *rotation;
        self.dirty_flag = true;
    }

    pub fn transform_position(&mut self, position: &Vector3) -> Vector3 {
        self.update_matrices();
        return self.position_matrix * (*position);
    }

    pub fn inverse_transform_position(&mut self, position: &Vector3) -> Vector3 {
        self.update_matrices();
        return self.inverse_position_matrix * (*position);
    }

    pub fn transform_direction(&mut self, direction: &Vector3) -> Vector3 {
        self.update_matrices();
        return self.direction_matrix * (*direction);
    }

    pub fn inverse_transform_direction(&mut self, direction: &Vector3) -> Vector3 {
        self.update_matrices();
        return self.inverse_direction_matrix * (*direction);
    }
    
    fn update_matrices(&mut self) {
        if !self.dirty_flag {
            return;
        }

        self.position_matrix = Matrix4::from(
            euler_rotation_matrix(&self.rotation)
        ) * translate_matrix(&self.position);

        self.inverse_position_matrix = translate_matrix(&-self.position) * Matrix4::from(
            inverse_euler_rotation_matrix(&self.rotation)
        );

        self.direction_matrix = Matrix3::from(self.position_matrix);
        
        self.inverse_direction_matrix = Matrix3::from(self.inverse_position_matrix);

        self.dirty_flag = false;
    }
}
