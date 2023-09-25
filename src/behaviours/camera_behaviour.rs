use std::cell::RefCell;
use std::rc::Rc;

use crate::ui::{ImguiEditor, drag_float3, drag_float};
use crate::utils::vector::{Vector3, FORWARD_VECTOR};
use crate::rendering::camera::Camera;

use super::Behaviour;

pub struct CameraBehaviour {
    target_camera: Rc<RefCell<Camera>>,
    rotation: Vector3,
    center: Vector3,
    distance: f32
}

impl CameraBehaviour {
    pub fn new(target_camera: Rc<RefCell<Camera>>) -> CameraBehaviour {
        CameraBehaviour {
            target_camera,
            rotation: Vector3::new(0.0, 0.0, 0.0),
            center: Vector3::new(0.0, 0.0, 0.0),
            distance: 2.0
        }
    }
}

impl Behaviour for CameraBehaviour {
    fn update(&mut self, _delta_time: f32) {
        let mut camera = self.target_camera.borrow_mut();
        let t = camera.get_mut_transform();
        t.set_rotation(&self.rotation);

        let position = self.center - t.transform_direction(&FORWARD_VECTOR) * self.distance;
        t.set_position(&position);
    }
}

impl ImguiEditor for CameraBehaviour {
    fn draw_ui(&mut self, ui: &imgui::Ui) {
        drag_float3(ui, "Rotation", &mut self.rotation);
        drag_float3(ui, "Center", &mut self.center);
        drag_float(ui, "Distance", &mut self.distance);
    }
}
