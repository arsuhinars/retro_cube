use std::{cell::RefCell, rc::Rc};

use crate::raycaster::{BoxRaycaster, SphereRaycaster, Raycaster};
use crate::rendering::renderer::Renderer;
use crate::utils::vector::{Vector3, ZERO_VECTOR};
use crate::ui::{ImguiEditor, drag_float3};

use super::Behaviour;

#[derive(PartialEq, Eq, Clone, Copy)]
enum RaycasterType {
    Box, Sphere
}

pub struct RaycasterBehaviour {
    renderer: Rc<RefCell<Renderer>>,
    raycaster_type: RaycasterType,
    is_static: bool,
    position: Vector3,
    rotation: Vector3,
    angular_velocity: Vector3
}

impl RaycasterBehaviour {
    pub fn new(renderer: Rc<RefCell<Renderer>>) -> RaycasterBehaviour {
        let mut behaviour = RaycasterBehaviour {
            renderer,
            raycaster_type: RaycasterType::Box,
            is_static: false,
            position: ZERO_VECTOR,
            rotation: ZERO_VECTOR,
            angular_velocity: Vector3::new(10.0, 10.0, 0.0)
        };

        behaviour.update_raycaster_type();

        behaviour
    }

    fn update_raycaster_type(&mut self) {
        let raycaster: Box<dyn Raycaster> =  match self.raycaster_type {
            RaycasterType::Box => Box::new(
                BoxRaycaster::new(&Vector3::new(1.0, 1.0, 1.0))
            ),
            RaycasterType::Sphere => Box::new(
                SphereRaycaster::new(0.5)
            )
        };
        
        self.renderer.borrow_mut().set_raycaster(raycaster);
    }
}

impl Behaviour for RaycasterBehaviour {
    fn update(&mut self, delta_time: f32) {
        if !self.is_static {
            self.rotation += self.angular_velocity * delta_time;
        }

        let mut renderer = self.renderer.borrow_mut();
        let raycaster = renderer.get_mut_raycaster();
        let transform = raycaster.get_mut_tranform();

        transform.set_position(&self.position);
        transform.set_rotation(&self.rotation);
    }
}

impl ImguiEditor for RaycasterBehaviour {
    fn draw_ui(&mut self, ui: &imgui::Ui) {
        ui.text("Raycaster type");
        if ui.radio_button("Box", &mut self.raycaster_type, RaycasterType::Box) {
            self.update_raycaster_type();
        }
        ui.same_line();
        if ui.radio_button("Sphere", &mut self.raycaster_type, RaycasterType::Sphere) {
            self.update_raycaster_type();
        }

        ui.checkbox("Is static", &mut self.is_static);
        drag_float3(ui, "Position", &mut self.position);
        if !self.is_static {
            drag_float3(ui, "Angular velocity", &mut self.angular_velocity);
        }
        ui.disabled(!self.is_static, || {
            drag_float3(ui, "Rotation", &mut self.rotation);
        });
    }
}
