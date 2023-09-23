// use sdl2::render::Texture;

use std::cell::{RefCell, Ref};

use crate::raycaster::{Raycaster, RaycastHit};
use crate::utils::{color::Color, vector::Vector3};
use super::camera::Camera;
use super::lightning::Lightning;
use super::material::Material;
use super::pixel_canvas::PixelCanvas;

pub struct Renderer {
    pixel_canvas: RefCell<PixelCanvas>,
    camera: Camera,
    raycaster: Box<dyn Raycaster>,
    material: Box<dyn Material>,
    lightning: Box<dyn Lightning>,
    // render_texture: RefCell<Texture>,
    // render_texture_size: (usize, usize)
}

impl Renderer {
    pub fn new(
        mut camera: Camera,
        raycaster: Box<dyn Raycaster>,
        material: Box<dyn Material>,
        lightning: Box<dyn Lightning>,
        pixel_canvas: PixelCanvas
        // render_texture: Texture
    ) -> Renderer {
        // let render_texture_size = query_texture_size(&render_texture);

        camera.set_aspect_ratio(pixel_canvas.get_aspect_ratio());

        Renderer {
            pixel_canvas: RefCell::new(pixel_canvas),
            camera,
            raycaster,
            material,
            lightning,
            // render_texture: RefCell::new(render_texture),
            // render_texture_size
        }
    }

    pub fn get_pixel_canvas(&self) -> Ref<PixelCanvas> {
        self.pixel_canvas.borrow()
    }

    pub fn set_pixel_canvas(&mut self, pixel_canvas: PixelCanvas) {
        self.pixel_canvas = RefCell::new(pixel_canvas);
        self.camera.set_aspect_ratio(self.pixel_canvas.borrow().get_aspect_ratio());
    }

    pub fn get_mut_camera(&mut self) -> &mut Camera {
        &mut self.camera
    }

    pub fn set_camera(&mut self, camera: Camera) {
        self.camera = camera;
        self.camera.set_aspect_ratio(self.pixel_canvas.borrow().get_aspect_ratio());
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

    // pub fn get_mut_render_texture(&mut self) -> &mut Texture {
    //     self.render_texture.get_mut()
    // }

    // pub fn set_render_texture(&mut self, render_texture: Texture) {
    //     let query = render_texture.query();
    //     let render_texture_size = (query.width as usize, query.height as usize);

    //     self.render_texture = RefCell::new(render_texture);
    //     self.render_texture_size = render_texture_size;
    //     self.camera.set_aspect_ratio(calc_aspect_ratio(self.render_texture_size));
    // }

    pub fn render(&mut self) {
        let width = self.pixel_canvas.borrow().get_width();
        let height = self.pixel_canvas.borrow().get_height();
        let w = width as f32;
        let h = height as f32;

        // self.render_texture.borrow_mut().with_lock(None, |data, pitch| {
        //     let pixel_pitch = pitch / width;

        //     for y in 0..height {
        //         for x in 0..width {
        //             let pixel = self.render_pixel(
        //                 (x as f32) / w * 2.0 - 1.0,
        //                 (h - (y as f32)) / h * 2.0 - 1.0
        //             );

        //             let idx = x * pixel_pitch + pitch * y;
        //             data[idx] = pixel.a;
        //             data[idx + 1] = pixel.b;
        //             data[idx + 2] = pixel.g;
        //             data[idx + 3] = pixel.r;
        //         }
        //     }
        // }).expect("Unable to render in texture");

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
        let (p, d) = self.camera.get_ray_origin_direction(clip_x, clip_y);

        match self.raycaster.raycast(&p, &d) {
            Some(hit) => self.compute_solid_color(&hit),
            None => self.compute_background_color(&d)
        }
    }

    fn compute_solid_color(&self, hit: &RaycastHit) -> Color {
        let base_color = self.material.compute_surface_color(&hit.local_position, &hit.local_normal);
        return self.lightning.apply_light(base_color, &hit.position, &hit.normal)
    }

    fn compute_background_color(&self, direction: &Vector3) -> Color {
        Color::new(0, 0, 0)
    }
}

// fn query_texture_size(texture: &Texture) -> (usize, usize) {
//     let q = texture.query();
//     (q.width as usize, q.height as usize)
// }

// fn calc_aspect_ratio(texture_size: (usize, usize)) -> f32 {
//     (texture_size.0 as f32) / (texture_size.1 as f32)
// }
