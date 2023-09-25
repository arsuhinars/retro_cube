use std::rc::Rc;

use glow::{NativeTexture, HasContext, TEXTURE_2D};
use imgui;

use crate::utils::color::Color;

pub struct PixelCanvas {
    gl: Rc<glow::Context>,
    texture: NativeTexture,
    imgui_texture_id: imgui::TextureId,
    width: usize,
    height: usize,
    data: Box<[u8]>
}

impl PixelCanvas {
    pub fn new(
        gl: Rc<glow::Context>,
        texture_map: &mut dyn imgui_glow_renderer::TextureMap,
        width: usize,
        height: usize
    ) -> Result<PixelCanvas, String> {
        let texture = unsafe { gl.create_texture()? };
        let imgui_texture_id = texture_map.register(texture).expect("Unable to register texture for imgui glow renderer");
        
        let mut pixel_canvas = PixelCanvas {
            gl,
            texture,
            imgui_texture_id,
            width,
            height,
            data: Box::new([])
        };

        pixel_canvas.update_texture();

        Ok(pixel_canvas)
    }

    pub fn update_with<F>(&mut self, f: F) where F: FnOnce(PixelWriter) -> () {
        f(PixelWriter { canvas: self });

        unsafe {
            self.gl.bind_texture(TEXTURE_2D, Some(self.texture));
            self.gl.tex_sub_image_2d(
                glow::TEXTURE_2D,
                0,
                0,
                0,
                self.width as i32,
                self.height as i32,
                glow::BGRA,
                glow::UNSIGNED_INT_8_8_8_8_REV,
                glow::PixelUnpackData::Slice(self.data.as_ref())
            );
            self.gl.bind_texture(TEXTURE_2D, None);
        }
    }

    pub fn render(&self, imgui: &imgui::Ui, screen_size: [f32; 2]) {
        imgui
            .get_background_draw_list()
            .add_image(self.imgui_texture_id, [0.0, 0.0], screen_size)
            .build();
    }

    pub fn get_width(&self) -> usize {
        self.width
    }

    pub fn get_height(&self) -> usize {
        self.height
    }

    pub fn get_aspect_ratio(&self) -> f32 {
        (self.width as f32) / (self.height as f32)
    }

    pub fn get_texture(&self) -> glow::NativeTexture {
        self.texture
    }

    pub fn get_imgui_texture_id(&self) -> imgui::TextureId {
        self.imgui_texture_id
    }

    pub fn resize(&mut self, width: usize, height: usize) {
        self.width = width;
        self.height = height;
        self.update_texture();
    }

    fn update_texture(&mut self) {
        let mut data_vec = Vec::<u8>::new();
        data_vec.resize(self.width * self.height * 4, 0);
        self.data = data_vec.into_boxed_slice();

        unsafe {
            self.gl.bind_texture(TEXTURE_2D, Some(self.texture));
            self.gl.tex_parameter_i32(TEXTURE_2D, glow::TEXTURE_BASE_LEVEL, 0);
            self.gl.tex_parameter_i32(TEXTURE_2D, glow::TEXTURE_MAX_LEVEL, 0);
            self.gl.tex_parameter_i32(TEXTURE_2D, glow::TEXTURE_MIN_FILTER, glow::NEAREST as i32);
            self.gl.tex_parameter_i32(TEXTURE_2D, glow::TEXTURE_MAG_FILTER, glow::NEAREST as i32);
            self.gl.tex_image_2d(
                TEXTURE_2D,
                0,
                glow::RGBA as i32,
                self.width as i32,
                self.height as i32,
                0,
                glow::BGRA,
                glow::UNSIGNED_INT_8_8_8_8_REV,
                Some(self.data.as_ref())
            );
            self.gl.bind_texture(TEXTURE_2D, None);
        }
    }
}

impl<'a> Drop for PixelCanvas {
    fn drop(&mut self) {
        unsafe { self.gl.delete_texture(self.texture); }
    }
}

pub struct PixelWriter<'a> {
    canvas: &'a mut PixelCanvas
}

impl<'a> PixelWriter<'a> {
    #[inline]
    pub fn write_pixel(&mut self, x: usize, y: usize, color: Color) {
        let idx = (x + y * self.canvas.width) * 4;
        self.canvas.data[idx] = color.b;
        self.canvas.data[idx + 1] = color.g;
        self.canvas.data[idx + 2] = color.r;
        self.canvas.data[idx + 3] = color.a;
    }
}
