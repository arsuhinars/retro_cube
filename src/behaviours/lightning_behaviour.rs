use std::{cell::RefCell, rc::Rc};

use crate::{utils::{color::Color, vector::Vector3}, rendering::{renderer::Renderer, lightning::{Lightning, UnlitLightning, DiffuseDirectLightning}}, ui::{ImguiEditor, drag_float3}};

use super::Behaviour;

#[derive(PartialEq, Eq, Clone, Copy)]
enum LightningType {
    Unlit, Lit
}

pub struct LightningBehaviour {
    renderer: Rc<RefCell<Renderer>>,
    lightning_type: LightningType,
    light_direction: Vector3,
    light_color: Color,
    ambient_color: Color
}

impl LightningBehaviour {
    pub fn new(renderer: Rc<RefCell<Renderer>>) -> LightningBehaviour {
        let mut behaviour = LightningBehaviour {
            renderer,
            lightning_type: LightningType::Lit,
            light_direction: Vector3::new(1.0, -1.0, 0.5),
            light_color: Color::new(255, 255, 255),
            ambient_color: Color::new(45, 45, 45)
        };

        behaviour.update_lightning();

        behaviour
    }

    fn update_lightning(&mut self) {
        let lightning: Box<dyn Lightning> = match self.lightning_type {
            LightningType::Unlit => Box::new(UnlitLightning::new(&self.ambient_color)),
            LightningType::Lit => Box::new(DiffuseDirectLightning::new(
                &self.light_direction,
                &self.light_color,
                &self.ambient_color)
            )
        };

        self.renderer.borrow_mut().set_lightning(lightning);
    }
}

impl Behaviour for LightningBehaviour {
    fn update(&mut self, delta_time: f32) {}
}

impl ImguiEditor for LightningBehaviour {
    fn draw_ui(&mut self, ui: &imgui::Ui) {
        let mut modified = false;

        ui.text("Lightning type");
        modified |= ui.radio_button(
            "Unlit",
            &mut self.lightning_type,
            LightningType::Unlit
        );
        ui.same_line();
        modified |= ui.radio_button(
            "Lit",
        &mut self.lightning_type,
            LightningType::Lit
        );
        
        match self.lightning_type {
            LightningType::Unlit => {
                modified |= ui.color_edit4("Ambient color", &mut self.ambient_color);
            },
            LightningType::Lit => {
                modified |= drag_float3(ui, "Light direction", &mut self.light_direction);
                modified |= ui.color_edit4("Light color", &mut self.light_color);
                modified |= ui.color_edit4("Ambient color", &mut self.ambient_color);
            }
        };

        if modified {
            self.update_lightning();
        }
    }
}
