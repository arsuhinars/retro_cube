use std::cell::RefCell;

use super::vector::Vector3;
use super::matrix::{Matrix3, Matrix4, translate_matrix, euler_rotation_matrix, inverse_euler_rotation_matrix};

#[derive(Clone)]
pub struct Transform {
    position: Vector3,
    rotation: Vector3,
    dirty_flag: RefCell<bool>,
    position_matrix: RefCell<Matrix4>,
    inverse_position_matrix: RefCell<Matrix4>,
    direction_matrix: RefCell<Matrix3>,
    inverse_direction_matrix: RefCell<Matrix3>
}

impl Transform {
    pub fn new(position: &Vector3, rotation: &Vector3) -> Transform {
        Transform {
            position: *position,
            rotation: *rotation,
            dirty_flag: RefCell::new(true),
            position_matrix: RefCell::new(Matrix4::default()),
            inverse_position_matrix: RefCell::new(Matrix4::default()),
            direction_matrix: RefCell::new(Matrix3::default()),
            inverse_direction_matrix: RefCell::new(Matrix3::default())
        }
    }

    pub fn get_position(&self) -> Vector3 { self.position }

    pub fn set_position(&mut self, position: &Vector3) {
        self.position = *position;
        self.dirty_flag.replace(true);
    }

    pub fn get_rotation(&self) -> Vector3 { self.rotation }

    pub fn set_rotation(&mut self, rotation: &Vector3) {
        self.rotation = *rotation;
        self.dirty_flag.replace(true);
    }

    pub fn transform_position(&self, position: &Vector3) -> Vector3 {
        self.update_matrices();
        return self.position_matrix.borrow().to_owned() * (*position);
    }

    pub fn inverse_transform_position(&self, position: &Vector3) -> Vector3 {
        self.update_matrices();
        return self.inverse_position_matrix.borrow().to_owned() * (*position);
    }

    pub fn transform_direction(&self, direction: &Vector3) -> Vector3 {
        self.update_matrices();
        return self.direction_matrix.borrow().to_owned() * (*direction);
    }

    pub fn inverse_transform_direction(&self, direction: &Vector3) -> Vector3 {
        self.update_matrices();
        return self.inverse_direction_matrix.borrow().to_owned() * (*direction);
    }
    
    fn update_matrices(&self) {
        if !self.dirty_flag.borrow().to_owned() {
            return;
        }

        self.position_matrix.replace(
            Matrix4::from(
                euler_rotation_matrix(&self.rotation)
            ) * translate_matrix(&self.position)
        );
        self.inverse_position_matrix.replace(
            translate_matrix(&-self.position) * Matrix4::from(
                inverse_euler_rotation_matrix(&self.rotation)
            )
        );
        self.direction_matrix.replace(
            Matrix3::from(self.position_matrix.borrow().to_owned())
        );
        self.inverse_direction_matrix.replace(
            Matrix3::from(self.inverse_position_matrix.borrow().to_owned())
        );

        self.dirty_flag.replace(false);
    }
}

impl Default for Transform {
    fn default() -> Self {
        Self {
            position: Vector3::default(),
            rotation: Vector3::default(),
            dirty_flag: RefCell::new(true),
            position_matrix: RefCell::default(),
            inverse_position_matrix: RefCell::default(),
            direction_matrix: RefCell::default(),
            inverse_direction_matrix: RefCell::default()
        }
    }
}
