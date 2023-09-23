use std::rc::Rc;

use sdl2;
use imgui_glow_renderer::{AutoRenderer as ImguiRenderer, TextureMap as ImguiTextureMap};
use glow::{self, HasContext};
use imgui;

use crate::rendering::camera::Camera;
use crate::rendering::lightning::{DiffuseDirectLightning, UnlitLightning};
use crate::rendering::material::{CheckerMaterial, SolidMaterial};
use crate::rendering::pixel_canvas::PixelCanvas;
use crate::rendering::renderer::Renderer;
use crate::raycaster::{SphereRaycaster, BoxRaycaster, Raycaster};
use crate::utils::{color::Color, vector::Vector3};

const APP_WINDOW_TITLE: &str = "retro_cube";
const APP_WINDOW_WIDTH: u32 = 640;
const APP_WINDOW_HEIGHT: u32 = 480;
const APP_RENDER_WIDTH: usize = 160;
const APP_RENDER_HEIGHT: usize = 120;

fn build_renderer(
    gl: Rc<glow::Context>, texture_map: &mut dyn ImguiTextureMap
) -> Result<Renderer, String> {
    let mut camera = Camera::default();
    camera.get_mut_transform().set_position(&Vector3::new(0.0, 0.0, -2.0));
    
    let mut raycaster = Box::new(
        BoxRaycaster::new(&Vector3::new(1.0, 1.0, 1.0))
    );
    raycaster.get_mut_tranform().set_rotation(
        &Vector3::new(45.0_f32.to_radians(), 45.0_f32.to_radians(), 0.0_f32.to_radians())
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

    let pixel_canvas = PixelCanvas::new(
        gl, texture_map, APP_RENDER_WIDTH, APP_RENDER_HEIGHT
    )?;

    // let render_texture = texture_creator.create_texture_streaming(
    //     sdl2::pixels::PixelFormatEnum::RGBA8888, APP_RENDER_WIDTH, APP_RENDER_HEIGHT
    // ).map_err(|err| err.to_string())?;

    Ok(
        Renderer::new(
            camera, raycaster, material, lightning, pixel_canvas
        )
    )
}

fn build_imgui(gl: glow::Context, window: &sdl2::video::Window) -> Result<(imgui::Context, ImguiRenderer), String> {
    let mut imgui = imgui::Context::create();
    imgui.set_ini_filename(None);
    imgui.set_log_filename(None);
    imgui.fonts().add_font(&[imgui::FontSource::DefaultFontData { config: None }]);

    let window_size = window.size();
    let window_drawable_size = window.drawable_size();
    let io = imgui.io_mut();
    io.display_size = [window_size.0 as f32, window_size.1 as f32];
    io.display_framebuffer_scale = [
        (window_drawable_size.0 as f32) / (window_size.0 as f32),
        (window_drawable_size.1 as f32) / (window_size.1 as f32),
    ];

    let renderer = ImguiRenderer::initialize(
        gl, &mut imgui
    ).map_err(|err| err.to_string())?;

    return Ok((imgui, renderer));
}

fn glow_context(window: &sdl2::video::Window) -> glow::Context {
    unsafe {
        glow::Context::from_loader_function(|s| window.subsystem().gl_get_proc_address(s) as _)
    }
}

pub struct App {
    sdl_context: sdl2::Sdl,
    video: sdl2::VideoSubsystem,
    timer: sdl2::TimerSubsystem,
    event_pump: sdl2::EventPump,
    canvas: sdl2::render::WindowCanvas,
    gl_context: sdl2::video::GLContext,
    // window: sdl2::video::Window,
    imgui: imgui::Context,
    imgui_renderer: ImguiRenderer,

    renderer: Renderer,
    is_running: bool,
    last_ticks: u32,
    delta_time: f32
}

impl App {
    pub fn init() -> Result<App, String> {
        let sdl_context = sdl2::init()?;
        let video = sdl_context.video()?;

        let gl_attr = video.gl_attr();
        gl_attr.set_context_version(3, 3);
        gl_attr.set_context_profile(sdl2::video::GLProfile::Core);

        let window = video.window(
            APP_WINDOW_TITLE, APP_WINDOW_WIDTH, APP_WINDOW_HEIGHT
        ).opengl().build().map_err(|err| err.to_string())?;

        let gl_context = window.gl_create_context()?;
        window.gl_make_current(&gl_context)?;
        window.subsystem().gl_set_swap_interval(1)?;

        let timer = sdl_context.timer()?;

        let canvas = window.into_canvas().build().map_err(|err| err.to_string())?;

        let gl = glow_context(canvas.window());

        let (imgui, mut imgui_renderer) = build_imgui(gl, canvas.window())?;

        let renderer = build_renderer(
            imgui_renderer.gl_context().clone(),
            imgui_renderer.texture_map_mut()
        )?;

        let event_pump = sdl_context.event_pump()?;

        return Ok(App {
            sdl_context,
            video,
            timer,
            event_pump,
            // window,
            gl_context,
            canvas,
            imgui,
            imgui_renderer,

            renderer,
            is_running: true,
            last_ticks: 0,
            delta_time: 0.0
        });
    }

    pub fn run(&mut self) {
        while self.is_running {
            self.delta_time = ((self.timer.ticks() - self.last_ticks) as f32) / 1000.0;
            self.last_ticks = self.timer.ticks();

            println!("{} fps", (1.0 / self.delta_time) as u32);

            self.handle_events();
            self.render();
        }
    }

    fn render(&mut self) {
        // self.canvas.set_draw_color(sdl2::pixels::Color::RGB(255, 255, 255));
        // self.canvas.clear();

        self.renderer.render();

        // self.canvas.copy(
        //     &self.renderer.get_mut_render_texture(), None, None
        // ).expect("unable to copy render texture on window");
        // unsafe { self.canvas.render_flush() };
        // self.canvas.present();

        let ui = self.imgui.new_frame();
        ui.show_demo_window(&mut true);

        unsafe { self.imgui_renderer.gl_context().clear(glow::COLOR_BUFFER_BIT) };

        self.renderer.get_pixel_canvas().render(
            ui,
            [APP_WINDOW_WIDTH as f32, APP_WINDOW_HEIGHT as f32]
        );

        let ui_draw_data = self.imgui.render();
        self.imgui_renderer.render(ui_draw_data).expect("unable to render ui");
        
        self.canvas.window().gl_swap_window();

        // self.canvas.present();
    }
    
    fn handle_events(&mut self) {
        for event in self.event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { timestamp: _ } => self.is_running = false,
                _ => ()
            }
        }
    }
}
