use std::{cell::RefCell, rc::Rc};

use crate::{rendering::renderer::Renderer, ui::ImguiEditor, app};

use super::Behaviour;

#[derive(PartialEq, Eq, Clone, Copy)]
enum RenderQuality {
    FullQuality, HalfQuality, QuarterQuality, OneEighthQuality
}

pub struct RendererBehaviour {
    renderer: Rc<RefCell<Renderer>>,
    render_quality: RenderQuality,
    delta_time: f32
}

impl RendererBehaviour {
    pub fn new(renderer: Rc<RefCell<Renderer>>) -> RendererBehaviour {
        let mut behaviour = RendererBehaviour {
            renderer,
            render_quality: RenderQuality::HalfQuality,
            delta_time: 0.0
        };

        behaviour.update_renderer();

        behaviour
    }

    fn update_renderer(&mut self) {
        let render_scale: f32 = match self.render_quality {
            RenderQuality::FullQuality => 1.0,
            RenderQuality::HalfQuality => 0.5,
            RenderQuality::QuarterQuality => 0.25,
            RenderQuality::OneEighthQuality => 0.125,
        };

        let render_width = ((app::APP_WINDOW_WIDTH as f32) * render_scale) as usize;
        let render_height = ((app::APP_WINDOW_HEIGHT as f32) * render_scale) as usize;

        self.renderer.borrow_mut()
            .get_mut_pixel_canvas()
            .resize(render_width, render_height);
    }
}

impl Behaviour for RendererBehaviour {
    fn update(&mut self, delta_time: f32) {
        self.delta_time = delta_time
    }
}

impl ImguiEditor for RendererBehaviour {
    fn draw_ui(&mut self, ui: &imgui::Ui) {
        ui.text(format!("{:.1} FPS", 1.0 / self.delta_time));

        ui.spacing();

        ui.text("Render scale");
        if ui.radio_button(
            "1x", &mut self.render_quality, RenderQuality::FullQuality
        ) {
            self.update_renderer();
        }
        ui.same_line();
        if ui.radio_button(
            "0.5x", &mut self.render_quality, RenderQuality::HalfQuality
        ) {
            self.update_renderer();
        }
        ui.same_line();
        if ui.radio_button(
            "0.25x", &mut self.render_quality, RenderQuality::QuarterQuality
        ) {
            self.update_renderer();
        }
        ui.same_line();
        if ui.radio_button(
            "0.125x", &mut self.render_quality, RenderQuality::OneEighthQuality
        ) {
            self.update_renderer();
        }
    }
}
