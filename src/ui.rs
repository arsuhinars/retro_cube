use imgui_glow_renderer::AutoRenderer as ImguiRenderer;
use imgui_sdl2_support::SdlPlatform as ImguiPlatform;

use crate::utils::vector::Vector3;

pub fn build_imgui(
    gl: glow::Context
) -> Result<(imgui::Context, ImguiPlatform, ImguiRenderer), String> {
    let mut imgui = imgui::Context::create();
    imgui.set_ini_filename(None);
    imgui.set_log_filename(None);
    imgui.fonts().add_font(&[imgui::FontSource::DefaultFontData { config: None }]);

    let platform = ImguiPlatform::init(&mut imgui);

    let renderer = ImguiRenderer::initialize(
        gl, &mut imgui
    ).map_err(|err| err.to_string())?;

    return Ok((imgui, platform, renderer));
}

pub trait ImguiEditor {
    fn draw_ui(&mut self, ui: &imgui::Ui);
}

const DEFAULT_DRAG_SPEED: f32 = 0.02;

pub fn drag_float(ui: &imgui::Ui, label: &str, value: &mut f32) -> bool {
    imgui::Drag::new(label).speed(DEFAULT_DRAG_SPEED).build(ui, value)
}

pub fn drag_float3(ui: &imgui::Ui, label: &str, value: &mut Vector3) -> bool {
    let mut arr = Into::<[f32; 3]>::into(*value);
    let result = imgui::Drag::new(label)
        .speed(DEFAULT_DRAG_SPEED)
        .build_array(ui, &mut arr);
    *value = Into::into(arr);

    return result;
}
