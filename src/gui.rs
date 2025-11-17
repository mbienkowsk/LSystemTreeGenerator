use egui_glium::egui_winit::egui::ViewportId;
use egui_glium::EguiGlium;
use glium::glutin::surface::WindowSurface;
use glium::Display;
use winit::event_loop::ActiveEventLoop;
use winit::window::Window;

pub struct GuiRenderer {
    pub egui_glium: EguiGlium,
}

impl GuiRenderer {
    pub fn new(
        display: &Display<WindowSurface>,
        window: &Window,
        event_loop: &ActiveEventLoop,
    ) -> Self {
        Self {
            egui_glium: EguiGlium::new(ViewportId::ROOT, display, window, event_loop),
        }
    }
}
