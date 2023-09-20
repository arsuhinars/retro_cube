use sdl2::render::Texture;
use super::{camera::Camera, render_pass::RenderPass};

pub struct Renderer {
    pub camera: Camera,
    render_passes: Box<[Box<dyn RenderPass>]>,
    render_texture: Texture,
    render_texture_size: (usize, usize)
}

impl Renderer {
    pub fn new(render_passes: Box<[Box<dyn RenderPass>]>, render_texture: Texture) -> Renderer {
        let query = render_texture.query();
        let render_texture_size = (query.width as usize, query.height as usize);
        
        let mut camera = Camera::default();
        camera.set_aspect_ratio((render_texture_size.0 as f32) / (render_texture_size.1 as f32));

        Renderer {
            camera,
            render_passes,
            render_texture,
            render_texture_size
        }
    }

    pub fn get_render_texture(&mut self) -> &mut Texture {
        &mut self.render_texture
    }

    pub fn render(&mut self) {
        for pass in self.render_passes.as_mut() {
            pass.as_mut().execute(
                &mut self.render_texture,
                self.render_texture_size,
                &mut self.camera
            )
        }
    }
}
