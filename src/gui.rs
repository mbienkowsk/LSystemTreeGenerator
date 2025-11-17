// rust
use egui::Context;
use egui_glium::EguiGlium;
use egui_glium::egui_winit::egui::ViewportId;
use glium::glutin::surface::WindowSurface;
use glium::{Display, Frame};
use log::info;
use winit::event::WindowEvent;
use winit::event_loop::ActiveEventLoop;
use winit::window::Window;

pub struct GuiController {
    egui_glium: EguiGlium,
    model_selection: ModelSelection,
    lsystem_config: LSystemConfig,
}

pub struct LSystemConfig {
    pub axiom: String,
    pub production_rules: Vec<(char, String)>,
    pub n_iterations: u32,
    pub angle: f32,
}

impl Default for LSystemConfig {
    fn default() -> Self {
        Self {
            axiom: "F".to_string(),
            production_rules: vec![('F', "F[+F]F[-F]F".to_string())],
            n_iterations: 3,
            angle: 25.0,
        }
    }
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
            lsystem_config: LSystemConfig::default(),
        }
    }

    pub fn get_model_selection(&self) -> &ModelSelection {
        &self.model_selection
    }

    pub fn get_lsystem_config(&self) -> &LSystemConfig {
        &self.lsystem_config
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
                .selected_text(format!("{model_selection:?}"))
                .show_ui(ui, |ui| {
                    ui.selectable_value(model_selection, ModelSelection::Monkey, "Monkey");
                    ui.selectable_value(model_selection, ModelSelection::Cone, "Cone");
                });
        });
    }

    fn ui_lsystem_config(
        lsystem_config: &mut LSystemConfig,
        ctx: &Context,
    ) {
        egui::Window::new("LSystem Configuration").show(ctx, |ui| {
            ui.add(egui::Slider::new(&mut lsystem_config.n_iterations, 0..=6).text("Number of Iterations"));
            ui.add(egui::Slider::new(&mut lsystem_config.angle, 0.0..=45.0).text("Angle"));
            ui.label(format!("{:?}", lsystem_config.axiom));
            ui.label("Production Rules:");
            for (i, (symbol, replacement)) in lsystem_config.production_rules.iter().enumerate() {
                ui.horizontal(|ui| {
                    ui.label(format!("{i}: {symbol} -> {replacement}"));
                });
            }
        });
    }

    pub fn draw(&mut self, window: &Window, display: &Display<WindowSurface>, frame: &mut Frame) {
        let model_selection = &mut self.model_selection;
        let lsystem_config = &mut self.lsystem_config;

        self.egui_glium.run(window, |ctx| {
            GuiController::ui_control_panel(model_selection, ctx);
            GuiController::ui_lsystem_config(lsystem_config, ctx);
        });
        self.egui_glium.paint(display, frame);
    }
}
