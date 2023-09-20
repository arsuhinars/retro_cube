use std::cell::RefCell;
use std::rc::Rc;

use sdl2::render::Texture;

use crate::raycaster::{Raycaster, RaycastHit};
use crate::utils::color::Color;
use super::camera::Camera;

pub trait RenderPass {
    fn execute(&mut self, target_texture: &mut Texture, texture_size: (usize, usize), camera: &mut Camera);
}

pub struct RaycastPass {
    raycaster: Rc<RefCell<dyn Raycaster>>
}

impl RaycastPass {
    pub fn new(raycaster: Rc<RefCell<dyn Raycaster>>) -> RaycastPass {
        RaycastPass { raycaster: raycaster.clone() }
    }

    fn compute_solid_color(&self, hit: &RaycastHit) -> Color {
        return Color::new(255, 255, 255);
    }

    fn compute_ambient_color(&self) -> Color {
        return Color::new(0, 0, 0);
    }
}

impl RenderPass for RaycastPass {
    fn execute(&mut self, target_texture: &mut Texture, texture_size: (usize, usize), camera: &mut Camera) {
        let width = texture_size.0 as usize;
        let height = texture_size.1 as usize;
        let w = width as f32;
        let h = height as f32;

        target_texture.with_lock(None, |data, pitch| {
            let pixel_pitch = pitch / width;

            for y in 0..height {
                for x in 0..width {
                    let clip_x = (x as f32) / w * 2.0 - 1.0;
                    let clip_y = (h - (y as f32)) / h * 2.0 - 1.0;

                    let (p, d) = camera.get_ray_origin_direction(clip_x, clip_y);

                    let result = match self.raycaster.borrow_mut().raycast(&p, &d) {
                        Some(hit) => self.compute_solid_color(&hit),
                        None => self.compute_ambient_color()
                    };

                    let idx = x * pixel_pitch + pitch * y;
                    data[idx] = result.a;
                    data[idx + 1] = result.b;
                    data[idx + 2] = result.g;
                    data[idx + 3] = result.r;
                }
            }
        }).expect("unable to lock texture for rendering in raycast render pass");
    }
}
