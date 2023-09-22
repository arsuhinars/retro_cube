use std::cell::RefCell;

use sdl2::render::Texture;

use crate::raycaster::{Raycaster, RaycastHit};
use crate::utils::{color::Color, vector::Vector3};
use super::camera::Camera;

pub struct Renderer {
    camera: Camera,
    raycaster: Box<dyn Raycaster>,
    render_texture: RefCell<Texture>,
    render_texture_size: (usize, usize)
}

impl Renderer {
    pub fn new(camera: Camera, raycaster: Box<dyn Raycaster>, render_texture: Texture) -> Renderer {
        let render_texture_size = query_texture_size(&render_texture);

        let mut renderer = Renderer {
            camera,
            raycaster,
            render_texture: RefCell::new(render_texture),
            render_texture_size
        };

        renderer.camera.set_aspect_ratio(calc_aspect_ratio(renderer.render_texture_size));

        return renderer;
    }

    pub fn get_mut_camera(&mut self) -> &mut Camera {
        &mut self.camera
    }

    pub fn set_camera(&mut self, camera: Camera) {
        self.camera = camera;
        self.camera.set_aspect_ratio(calc_aspect_ratio(self.render_texture_size));
    }

    pub fn get_mut_raycaster(&mut self) -> &mut dyn Raycaster {
        self.raycaster.as_mut()
    }

    pub fn set_raycaster(&mut self, raycaster: Box<dyn Raycaster>) {
        self.raycaster = raycaster;
    }

    pub fn get_mut_render_texture(&mut self) -> &mut Texture {
        self.render_texture.get_mut()
    }

    pub fn set_render_texture(&mut self, render_texture: Texture) {
        let query = render_texture.query();
        let render_texture_size = (query.width as usize, query.height as usize);

        self.render_texture = RefCell::new(render_texture);
        self.render_texture_size = render_texture_size;
        self.camera.set_aspect_ratio(calc_aspect_ratio(self.render_texture_size));
    }

    pub fn render(&mut self) {
        let width = self.render_texture_size.0 as usize;
        let height = self.render_texture_size.1 as usize;
        let w = width as f32;
        let h = height as f32;

        self.render_texture.borrow_mut().with_lock(None, |data, pitch| {
            let pixel_pitch = pitch / width;

            for y in 0..height {
                for x in 0..width {
                    let pixel = self.render_pixel(
                        (x as f32) / w * 2.0 - 1.0,
                        (h - (y as f32)) / h * 2.0 - 1.0
                    );

                    let idx = x * pixel_pitch + pitch * y;
                    data[idx] = pixel.a;
                    data[idx + 1] = pixel.b;
                    data[idx + 2] = pixel.g;
                    data[idx + 3] = pixel.r;
                }
            }
        }).expect("Unable to render in texture");
    }

    fn render_pixel(&self, clip_x: f32, clip_y: f32) -> Color {
        let (p, d) = self.camera.get_ray_origin_direction(clip_x, clip_y);

        match self.raycaster.raycast(&p, &d) {
            Some(hit) => self.compute_solid_color(&hit),
            None => self.compute_background_color(&d)
        }
    }

    fn compute_solid_color(&self, hit: &RaycastHit) -> Color {
        Color::new(255, 255, 255)
    }

    fn compute_background_color(&self, direction: &Vector3) -> Color {
        Color::new(0, 0, 0)
    }
}

fn query_texture_size(texture: &Texture) -> (usize, usize) {
    let q = texture.query();
    (q.width as usize, q.height as usize)
}

fn calc_aspect_ratio(texture_size: (usize, usize)) -> f32 {
    (texture_size.0 as f32) / (texture_size.1 as f32)
}
