use std::{cell::RefCell, rc::Rc};

use crate::{utils::color::Color, rendering::{renderer::Renderer, material::{FlatMaterial, CheckerMaterial, Material}}, ui::{ImguiEditor, drag_float}};

use super::Behaviour;

#[derive(PartialEq, Eq, Clone, Copy)]
enum MaterialType {
    Flat, Checker
}

pub struct MaterialBehaviour {
    renderer: Rc<RefCell<Renderer>>,
    material_type: MaterialType,
    first_color: Color,
    second_color: Color,
    scale: f32
}

impl MaterialBehaviour {
    pub fn new(renderer: Rc<RefCell<Renderer>>) -> MaterialBehaviour {
        let mut behaviour = MaterialBehaviour {
            renderer,
            material_type: MaterialType::Checker,
            first_color: Color::new(255, 255, 255),
            second_color: Color::new(127, 127, 127),
            scale: 1.0
        };

        behaviour.update_material();

        behaviour
    }

    fn update_material(&mut self) {
        let material: Box<dyn Material> = match self.material_type {
            MaterialType::Flat => Box::new(FlatMaterial { color: self.first_color }),
            MaterialType::Checker => Box::new(
                CheckerMaterial {
                    scale: self.scale,
                    first_color: self.first_color,
                    second_color: self.second_color
                }
            )
        };
        self.renderer.borrow_mut().set_material(material);
    }
}

impl Behaviour for MaterialBehaviour {
    fn update(&mut self, _delta_time: f32) {}
}

impl ImguiEditor for MaterialBehaviour {
    fn draw_ui(&mut self, ui: &imgui::Ui) {
        let mut modified: bool = false;

        ui.text("Material type");
        modified |= ui.radio_button(
            "Flat",
            &mut self.material_type,
            MaterialType::Flat
        );
        ui.same_line();
        modified |= ui.radio_button(
            "Checker",
            &mut self.material_type,
            MaterialType::Checker
        );

        match self.material_type {
            MaterialType::Flat => {
                modified |= ui.color_edit4("Color", &mut self.first_color);
            },
            MaterialType::Checker => {
                modified |= ui.color_edit4("First color", &mut self.first_color);
                modified |= ui.color_edit4("Second color", &mut self.second_color);
                modified |= drag_float(ui, "Scale", &mut self.scale);
            }
        };

        if modified {
            self.update_material();
        }
    }
}
