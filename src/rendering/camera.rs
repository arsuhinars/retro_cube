use std::cell::RefCell;

use crate::utils::{transform::Transform, vector::Vector3, matrix::Matrix3};

pub struct Camera {
    transform: Transform,
    ray_matrix: RefCell<Matrix3>,
    dirty_flag: RefCell<bool>,
    fov: f32,
    aspect_ratio: f32
}

impl Camera {
    pub fn get_mut_transform(&mut self) -> &mut Transform {
        &mut self.transform
    }

    pub fn get_fov(&self) -> f32 { self.fov }

    pub fn set_fov(&mut self, value: f32) {
        self.fov = value;
        self.dirty_flag.replace(true);
    }

    pub fn get_aspect_ratio(&self) -> f32 { self.aspect_ratio }

    pub fn set_aspect_ratio(&mut self, value: f32) {
        self.aspect_ratio = value;
        self.dirty_flag.replace(true);
    }

    /// Get origin and direction vectors (sequentially in a tuple)
    /// for a ray emitted from given pixel coordinates
    /// ((-1, -1) is left-bottom corner, (1, 1) is right-top corner).
    pub fn get_ray_origin_direction(&self, x: f32, y: f32) -> (Vector3, Vector3) {
        if self.dirty_flag.borrow().to_owned() {
            let k = (self.fov * 0.5).to_radians().tan();

            self.ray_matrix.replace(
                Matrix3([
                    [k * self.aspect_ratio, 0.0, 0.0],
                    [0.0, k, 0.0],
                    [0.0, 0.0, 1.0]
                ])
            );

            self.dirty_flag.replace(false);
        }

        let local_dir = self.ray_matrix.borrow().to_owned() * Vector3::new(x, y, 1.0);

        return (self.transform.get_position(), self.transform.transform_direction(&local_dir));
    }
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            transform: Default::default(),
            ray_matrix: Default::default(),
            dirty_flag: RefCell::new(true),
            fov: 60.0,
            aspect_ratio: 1.0
        }
    }
}
