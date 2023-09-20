use std::cell::RefCell;
use std::rc::Rc;

use sdl2::pixels::PixelFormatEnum;
use sdl2::render::TextureCreator;
use sdl2::video::WindowContext;
use sdl2::{Sdl, VideoSubsystem, EventPump, TimerSubsystem, render::WindowCanvas, event::Event, pixels::Color};

use crate::rendering::render_pass::RenderPass;
use crate::rendering::{renderer::Renderer, render_pass::RaycastPass};
use crate::raycaster::{SphereRaycaster, Raycaster, BoxRaycaster};
use crate::utils::vector::Vector3;

const APP_WINDOW_TITLE: &str = "retro_cube";
const APP_WINDOW_WIDTH: u32 = 640;
const APP_WINDOW_HEIGHT: u32 = 480;
const APP_RENDER_WIDTH: u32 = 320;
const APP_RENDER_HEIGHT: u32 = 240;

fn build_renderer(
    raycaster: Rc<RefCell<dyn Raycaster>>, texture_creator: TextureCreator<WindowContext>
) -> Result<Renderer, String> {
    let mut passes = Vec::<Box<dyn RenderPass>>::new();
    passes.push(Box::new(RaycastPass::new(raycaster)));

    let mut renderer = Renderer::new(
        passes.into_boxed_slice(),
        texture_creator.create_texture_streaming(
            PixelFormatEnum::RGBA8888, APP_RENDER_WIDTH, APP_RENDER_HEIGHT
        ).map_err(|err| err.to_string())?
    );

    renderer.camera.get_mut_transform().set_position(&Vector3::new(0.0, 0.0, -2.0));
    // renderer.camera.get_mut_transform().set_rotation(&Vector3::new(0.0, -30.0_f32.to_radians(), 0.0));

    Ok(renderer)
}

pub struct App {
    sdl_context: Sdl,
    video: VideoSubsystem,
    timer: TimerSubsystem,
    event_pump: EventPump,
    canvas: WindowCanvas,

    raycaster: Rc<RefCell<dyn Raycaster>>,
    renderer: Renderer,
    
    is_running: bool
}

impl App {
    pub fn init() -> Result<App, String> {
        let sdl_context = sdl2::init()?;
        let video = sdl_context.video()?;
        let timer = sdl_context.timer()?;
        let event_pump = sdl_context.event_pump()?;
        let window = video.window(
            APP_WINDOW_TITLE, APP_WINDOW_WIDTH, APP_WINDOW_HEIGHT
        ).build().map_err(|err| err.to_string())?;
        let canvas = window.into_canvas().build().map_err(|err| err.to_string())?;

        let raycaster = Rc::new(RefCell::new(
            BoxRaycaster::new(&Vector3::new(0.5, 0.5, 0.5)))
        );
        raycaster.borrow_mut().get_mut_tranform().set_rotation(
            &Vector3::new(0.0_f32.to_radians(), 40.0_f32.to_radians(), 25.0_f32.to_radians())
        );
        let renderer = build_renderer(
            raycaster.clone(), canvas.texture_creator()
        )?;

        return Ok(App {
            sdl_context,
            video,
            timer,
            event_pump,
            canvas,

            renderer,
            raycaster,

            is_running: true,
        });
    }

    pub fn run(&mut self) {
        while self.is_running {
            self.handle_events();
            self.render();
        }
    }

    fn render(&mut self) {
        self.canvas.set_draw_color(Color::RGB(255, 255, 255));
        self.canvas.clear();

        self.renderer.render();
        
        self.canvas.copy(
            &self.renderer.get_render_texture(), None, None
        ).expect("unable to copy render texture on window");
        
        self.canvas.present();
    }
    
    fn handle_events(&mut self) {
        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit { timestamp: _ } => self.is_running = false,
                _ => ()
            }
        }
    }
}
