use imgui_glow_renderer::AutoRenderer as ImguiRenderer;
use imgui_sdl2_support::SdlPlatform as ImguiPlatform;

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
