use egui_glium::egui_winit::egui::ViewportId;
use egui_glium::EguiGlium;
use glium::glutin::surface::WindowSurface;
use glium::Display;
use winit::event::WindowEvent;
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

    pub fn handle_event(&mut self, event: &WindowEvent, window: &Window) {
        let _ = self.egui_glium.on_event(window, event);
    }

    pub fn draw_ui(&mut self, display: &Display<WindowSurface>, window: &Window) {
        self.egui_glium.run(window, |ctx| {
            egui::SidePanel::left("my_side_panel").show(ctx, |ui| {
                ui.heading("Hello World!");
                if ui.button("Click").clicked() {
                    log::info!("Clicked button")
                }
            });

        })
    }
}
