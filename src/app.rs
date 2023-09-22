use sdl2;

use crate::rendering::camera::Camera;
use crate::rendering::lightning::{DiffuseDirectLightning, UnlitLightning};
use crate::rendering::material::{CheckerMaterial, SolidMaterial};
use crate::rendering::renderer::Renderer;
use crate::raycaster::{SphereRaycaster, BoxRaycaster, Raycaster};
use crate::utils::{color::Color, vector::Vector3};

const APP_WINDOW_TITLE: &str = "retro_cube";
const APP_WINDOW_WIDTH: u32 = 640;
const APP_WINDOW_HEIGHT: u32 = 480;
const APP_RENDER_WIDTH: u32 = 160;
const APP_RENDER_HEIGHT: u32 = 120;

fn build_renderer(
    texture_creator: sdl2::render::TextureCreator<sdl2::video::WindowContext>
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

    let render_texture = texture_creator.create_texture_streaming(
        sdl2::pixels::PixelFormatEnum::RGBA8888, APP_RENDER_WIDTH, APP_RENDER_HEIGHT
    ).map_err(|err| err.to_string())?;

    Ok(
        Renderer::new(
            camera, raycaster, material, lightning, render_texture
        )
    )
}

pub struct App {
    sdl_context: sdl2::Sdl,
    video: sdl2::VideoSubsystem,
    timer: sdl2::TimerSubsystem,
    event_pump: sdl2::EventPump,
    canvas: sdl2::render::WindowCanvas,

    renderer: Renderer,
    is_running: bool,
    last_ticks: u32,
    delta_time: f32
}

impl App {
    pub fn init() -> Result<App, String> {
        let sdl_context = sdl2::init()?;
        let video = sdl_context.video()?;
        let timer = sdl_context.timer()?;
        let event_pump = sdl_context.event_pump()?;
        let window = video.window(
            APP_WINDOW_TITLE, APP_WINDOW_WIDTH, APP_WINDOW_HEIGHT
        ).opengl().build().map_err(|err| err.to_string())?;

        let gl_context = window.gl_create_context()?;
        window.gl_make_current(&gl_context)?;
        video.gl_set_swap_interval(1)?;

        let canvas = window.into_canvas().build().map_err(|err| err.to_string())?;

        let renderer = build_renderer(canvas.texture_creator())?;

        return Ok(App {
            sdl_context,
            video,
            timer,
            event_pump,
            canvas,

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
        self.canvas.set_draw_color(sdl2::pixels::Color::RGB(255, 255, 255));
        self.canvas.clear();

        self.renderer.render();
        
        self.canvas.copy(
            &self.renderer.get_mut_render_texture(), None, None
        ).expect("unable to copy render texture on window");

        self.canvas.present();
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
