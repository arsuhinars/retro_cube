use std::cell::{RefCell, Ref, RefMut};
use std::rc::Rc;

use imgui_glow_renderer::TextureMap as ImguiTextureMap;

use crate::raycaster::{Raycaster, RaycastHit, BoxRaycaster};
use crate::utils::{color::Color, vector::Vector3};

use super::camera::Camera;
use super::lightning::{Lightning, DiffuseDirectLightning};
use super::material::{Material, CheckerMaterial};
use super::pixel_canvas::PixelCanvas;

pub struct Renderer {
    pixel_canvas: RefCell<PixelCanvas>,
    camera: Rc<RefCell<Camera>>,
    raycaster: Box<dyn Raycaster>,
    material: Box<dyn Material>,
    lightning: Box<dyn Lightning>
}

impl Renderer {
    pub fn get_pixel_canvas(&self) -> Ref<PixelCanvas> {
        self.pixel_canvas.borrow()
    }

    pub fn get_mut_pixel_canvas(&self) -> RefMut<PixelCanvas> {
        self.pixel_canvas.borrow_mut()
    }

    pub fn get_camera(&self) -> Rc<RefCell<Camera>> {
        self.camera.clone()
    }

    pub fn get_mut_raycaster(&mut self) -> &mut dyn Raycaster {
        self.raycaster.as_mut()
    }

    pub fn set_raycaster(&mut self, raycaster: Box<dyn Raycaster>) {
        self.raycaster = raycaster;
    }

    pub fn get_mut_material(&mut self) -> &mut dyn Material {
        self.material.as_mut()
    }

    pub fn set_material(&mut self, material: Box<dyn Material>) {
        self.material = material;
    }

    pub fn get_mut_lightning(&mut self) -> &mut dyn Lightning {
        self.lightning.as_mut()
    }

    pub fn set_lightning(&mut self, lightning: Box<dyn Lightning>) {
        self.lightning = lightning;
    }

    pub fn render(&mut self) {
        let width = self.pixel_canvas.borrow().get_width();
        let height = self.pixel_canvas.borrow().get_height();
        let w = width as f32;
        let h = height as f32;

        self.pixel_canvas.borrow_mut().update_with(|mut writer| {
            for y in 0..height {
                for x in 0..width {
                    let pixel = self.render_pixel(
                        (x as f32) / w * 2.0 - 1.0,
                        (h - (y as f32)) / h * 2.0 - 1.0
                    );

                    writer.write_pixel(x, y, pixel);
                }
            }
        });
    }

    fn render_pixel(&self, clip_x: f32, clip_y: f32) -> Color {
        let (p, d) = self.camera.borrow().get_ray_origin_direction(clip_x, clip_y);

        match self.raycaster.raycast(&p, &d) {
            Some(hit) => self.compute_solid_color(&hit),
            None => self.compute_background_color(&d)
        }
    }

    fn compute_solid_color(&self, hit: &RaycastHit) -> Color {
        let base_color = self.material.compute_surface_color(&hit.local_position, &hit.local_normal);
        return self.lightning.apply_light(base_color, &hit.position, &hit.normal)
    }

    fn compute_background_color(&self, _direction: &Vector3) -> Color {
        Color::new(0, 0, 0)
    }
}

pub fn build_renderer(
    gl: Rc<glow::Context>, texture_map: &mut dyn ImguiTextureMap, render_size: [usize; 2]
) -> Result<Renderer, String> {
    let mut camera = Camera::default();
    camera.get_mut_transform().set_position(&Vector3::new(0.0, 0.0, -2.0));
    camera.set_aspect_ratio((render_size[0] as f32) / (render_size[1] as f32));
    
    let raycaster = Box::new(
        BoxRaycaster::new(&Vector3::new(1.0, 1.0, 1.0))
    );

    let material = Box::new(CheckerMaterial { 
        scale: 1.0,
        first_color: Color::new(255, 255, 255),
        second_color: Color::new(127, 127, 127)
    });

    let lightning = Box::new(
        DiffuseDirectLightning::new(
            &Vector3::new(1.0, -1.0, 1.0),
            &Color::new(255, 255, 255),
            &Color::new(40, 40, 40)
        )
    );

    let pixel_canvas = RefCell::new(
        PixelCanvas::new(
            gl, texture_map, render_size[0], render_size[1]
        )?
    );

    let renderer = Renderer {
        pixel_canvas,
        camera: Rc::new(RefCell::new(camera)),
        raycaster,
        material,
        lightning
    };

    Ok(renderer)
}
