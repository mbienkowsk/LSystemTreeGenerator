// rust
use egui::Context;
use egui_glium::egui_winit::egui::ViewportId;
use egui_glium::EguiGlium;
use glium::glutin::surface::WindowSurface;
use glium::{Display, Frame};
use log::info;
use winit::event::WindowEvent;
use winit::event_loop::ActiveEventLoop;
use winit::window::Window;

pub struct GuiController {
    egui_glium: EguiGlium,
    model_selection: ModelSelection,
    axiom: String,
    production_rules: Vec<(char, String)>,
    n_iterations: u32,
    angle: f32,
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
        let axiom = "F".to_string();
        let production_rules = vec![('F', "F[+F]F[-F]F".to_string())];
        let n_iterations = 3;
        let angle = 25.0;

        Self {
            egui_glium: EguiGlium::new(ViewportId::ROOT, display, window, event_loop),
            model_selection: ModelSelection::Monkey,
            axiom,
            production_rules,
            n_iterations,
            angle,
        }
    }

    pub fn get_model_selection(&self) -> &ModelSelection {
        &self.model_selection
    }

    pub fn get_axiom(&self) -> &str {
        &self.axiom
    }

    pub fn get_n_iterations(&self) -> u32 {
        self.n_iterations
    }

    pub fn get_angle(&self) -> f32 {
        self.angle
    }

    pub fn get_production_rules(&self) -> &Vec<(char, String)> {
        &self.production_rules
    }

    pub fn handle_event(&mut self, event: &WindowEvent, window: &Window) {
        let _ = self.egui_glium.on_event(window, event);
    }

    fn ui_control_panel(model_selection: &mut ModelSelection, ctx: &Context) {
        egui::Window::new("Control panel").show(ctx, |ui| {
            ui.heading("Hello World!");
            if ui.button("Click").clicked() {
                info!("Clicked button");
            }
            egui::ComboBox::from_label("Selected Model")
                .selected_text(format!("{:?}", model_selection))
                .show_ui(ui, |ui| {
                    ui.selectable_value(model_selection, ModelSelection::Monkey, "Monkey");
                    ui.selectable_value(model_selection, ModelSelection::Cone, "Cone");
                });
        });
    }

    fn ui_lsystem_config(axiom: &str, production_rules: &[(char, String)], n_iterations: &mut u32, ctx: &Context) {
        egui::Window::new("LSystem Configuration").show(ctx, |ui| {
            ui.label(format!("{:?}", axiom));
            ui.label("Production Rules:");
            for (i, (symbol, replacement)) in production_rules.iter().enumerate() {
                ui.horizontal(|ui| {
                    ui.label(format!("{}: {} -> {}", i, symbol, replacement));
                });
            }
        });
    }

    pub fn draw(&mut self, window: &Window, display: &Display<WindowSurface>, frame: &mut Frame) {
        let model_selection = &mut self.model_selection;
        let axiom = &self.axiom;
        let production_rules = &self.production_rules;
        let n_iterations = &mut self.n_iterations;

        self.egui_glium.run(window, |ctx| {
            GuiController::ui_control_panel(model_selection, ctx);
            GuiController::ui_lsystem_config(axiom, production_rules, n_iterations, ctx);
        });
        self.egui_glium.paint(display, frame);
    }
}
