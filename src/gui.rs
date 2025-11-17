use egui_glium::EguiGlium;
use egui_glium::egui_winit::egui::ViewportId;
use glium::{Display, Frame};
use glium::glutin::surface::WindowSurface;
use log::info;
use winit::event::WindowEvent;
use winit::event_loop::ActiveEventLoop;
use winit::window::Window;

pub struct GuiController {
    egui_glium: EguiGlium,
    model_selection: ModelSelection,
}

#[derive(Debug, PartialEq)]
pub enum ModelSelection {
    Monkey,
    Cone,
}

impl GuiController {
    pub fn new(
        display: &Display<WindowSurface>,
        window: &Window,
        event_loop: &ActiveEventLoop,
    ) -> Self {
        Self {
            egui_glium: EguiGlium::new(ViewportId::ROOT, display, window, event_loop),
            model_selection: ModelSelection::Monkey,
        }
    }

    pub fn get_model_selection(&self) -> &ModelSelection {
        &self.model_selection
    }

    pub fn handle_event(&mut self, event: &WindowEvent, window: &Window) {
        let _ = self.egui_glium.on_event(window, event);
    }

    fn ui_components(&mut self, window: &Window) {
        self.egui_glium.run(window, |ctx| {
            egui::Window::new("Control panel").show(ctx, |ui| {
                ui.heading("Hello World!");
                if ui.button("Click").clicked() {
                    info!("Clicked button");
                }

                egui::ComboBox::from_label("Selected Model")
                    .selected_text(format!("{:?}", self.model_selection))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(
                            &mut self.model_selection,
                            ModelSelection::Monkey,
                            "Monkey",
                        );
                        ui.selectable_value(
                            &mut self.model_selection,
                            ModelSelection::Cone,
                            "Cone",
                        );
                    })
            });
        });
    }
    
    pub fn draw(&mut self, window: &Window, display: &Display<WindowSurface>, frame: &mut Frame) {
        self.ui_components(window);
        self.egui_glium.paint(display, frame);
    }
}
