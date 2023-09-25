use std::{time::Instant, cell::RefCell, rc::Rc};

use sdl2::{
    VideoSubsystem,
    EventPump,
    event::Event,
    video::{Window, GLContext, GLProfile},
};
use imgui_glow_renderer::AutoRenderer as ImguiRenderer;
use imgui_sdl2_support::SdlPlatform as ImguiPlatform;
use glow::HasContext;
use imgui;

use crate::{rendering::renderer::{Renderer, build_renderer}, ui::ImguiEditor, behaviours::renderer_behaviour::{self, RendererBehaviour}};
use crate::behaviours::{camera_behaviour::CameraBehaviour, Behaviour};
use crate::ui::build_imgui;

const APP_WINDOW_TITLE: &str = "retro_cube";
pub const APP_WINDOW_WIDTH: u32 = 640;
pub const APP_WINDOW_HEIGHT: u32 = 480;
// const APP_RENDER_WIDTH: usize = 160;
// const APP_RENDER_HEIGHT: usize = 120;

fn glow_context(window: &sdl2::video::Window) -> glow::Context {
    unsafe {
        glow::Context::from_loader_function(
            |s| window.subsystem().gl_get_proc_address(s) as _
        )
    }
}

pub struct App {
    video: VideoSubsystem,
    event_pump: EventPump,
    gl_context: GLContext,
    window: Window,

    imgui: imgui::Context,
    imgui_platform: ImguiPlatform,
    imgui_renderer: ImguiRenderer,

    renderer: Rc<RefCell<Renderer>>,
    camera_behaviour: CameraBehaviour,
    renderer_behaviour: RendererBehaviour,
    
    is_running: bool,
    time_instant: Instant,
    delta_time: f32
}

impl App {
    pub fn init() -> Result<App, String> {
        let sdl_context = sdl2::init()?;
        let video = sdl_context.video()?;
        let event_pump = sdl_context.event_pump()?;

        let gl_attr = video.gl_attr();
        gl_attr.set_context_version(4, 1);
        gl_attr.set_context_profile(GLProfile::Core);

        let window = video.window(
            APP_WINDOW_TITLE, APP_WINDOW_WIDTH, APP_WINDOW_HEIGHT
        ).opengl().build().map_err(|err| err.to_string())?;

        let gl_context = window.gl_create_context()?;
        window.gl_make_current(&gl_context)?;
        video.gl_set_swap_interval(1)?;

        let gl = glow_context(&window);

        let (imgui, imgui_platform, mut imgui_renderer) = build_imgui(gl)?;
        
        let renderer = Rc::new(RefCell::new(build_renderer(
            imgui_renderer.gl_context().clone(),
            imgui_renderer.texture_map_mut(),
            [APP_WINDOW_WIDTH as usize, APP_WINDOW_HEIGHT as usize]
        )?));
        let camera_behaviour = CameraBehaviour::new(renderer.borrow().get_camera().clone());
        let renderer_behaviour = RendererBehaviour::new(renderer.clone());

        return Ok(App {
            video,
            event_pump,
            gl_context,
            window,

            imgui,
            imgui_platform,
            imgui_renderer,

            renderer,
            camera_behaviour,
            renderer_behaviour,

            is_running: true,
            time_instant: Instant::now(),
            delta_time: 0.0
        });
    }

    pub fn run(&mut self) {
        while self.is_running {
            self.delta_time = self.time_instant.elapsed().as_secs_f32();
            self.time_instant = Instant::now();

            self.handle_events();
            self.render();
        }
    }

    fn render(&mut self) {
        self.imgui_platform.prepare_frame(&mut self.imgui, &self.window, &self.event_pump);

        let ui = self.imgui.new_frame();
        App::build_window(ui, || {
            if ui.collapsing_header("Rendering", imgui::TreeNodeFlags::empty()) {
                self.renderer_behaviour.draw_ui(ui);
            }

            if ui.collapsing_header("Camera", imgui::TreeNodeFlags::empty()) {
                self.camera_behaviour.draw_ui(ui);
            }

            if ui.collapsing_header("Raycaster", imgui::TreeNodeFlags::empty()) {
                
            }

            if ui.collapsing_header("Material", imgui::TreeNodeFlags::empty()) {
                
            }

            if ui.collapsing_header("Lightning", imgui::TreeNodeFlags::empty()) {
                
            }
        });

        self.renderer.borrow_mut().render();
        self.camera_behaviour.update(self.delta_time);
        self.renderer_behaviour.update(self.delta_time);

        self.renderer.borrow().get_pixel_canvas().render(
            ui,
            [APP_WINDOW_WIDTH as f32, APP_WINDOW_HEIGHT as f32]
        );

        let ui_draw_data = self.imgui.render();
        unsafe { self.imgui_renderer.gl_context().clear(glow::COLOR_BUFFER_BIT) };
        self.imgui_renderer.render(ui_draw_data).expect("unable to render UI");

        self.window.gl_swap_window();
    }
    
    fn handle_events(&mut self) {
        for event in self.event_pump.poll_iter() {
            self.imgui_platform.handle_event(&mut self.imgui, &event);

            match event {
                Event::Quit { timestamp: _ } => self.is_running = false,
                _ => ()
            }
        }
    }

    fn build_window<F>(ui: &imgui::Ui, f: F) where F: FnOnce() -> () {
        ui.window("Inspector")
            .size(
                [(APP_WINDOW_WIDTH as f32) * 0.45, APP_WINDOW_HEIGHT as f32],
                imgui::Condition::Once
            )
            .size_constraints([0.0, -1.0], [f32::INFINITY, -1.0])
            .position([0.0, 0.0], imgui::Condition::Always)
            .movable(false)
            .build(f);
    }
}
