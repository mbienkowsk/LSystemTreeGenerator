use egui::Ui;
use egui_glium::egui_winit::egui::ViewportId;
use egui_glium::EguiGlium;
use glium::glutin::surface::WindowSurface;
use glium::{Display, Frame};
use winit::event::WindowEvent;
use winit::event_loop::ActiveEventLoop;
use winit::window::Window;

pub struct GuiController {
    egui_glium: EguiGlium,
    model_selection: ModelSelection,
    lsystem_config: LSystemConfig,
    shading_mode: ShadingMode,
    interpolation_color_low: [f32; 3],
    interpolation_color_high: [f32; 3],
}

#[derive(Debug, Clone, PartialEq)]
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
            production_rules: vec![('F', "F[+F][&F][\\F]F[-F][^F][/F]F".to_string())],
            n_iterations: 3,
            angle: 25.0,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum ModelSelection {
    Cylinder,
    Monkey,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ShadingMode {
    Flat,
    Gouraud,
    Phong,
}

impl From<ShadingMode> for i32 {
    fn from(mode: ShadingMode) -> Self {
        match mode {
            ShadingMode::Flat => 0,
            ShadingMode::Gouraud => 1,
            ShadingMode::Phong => 2,
        }
    }
}

impl GuiController {
    pub fn new(
        display: &Display<WindowSurface>,
        window: &Window,
        event_loop: &ActiveEventLoop,
    ) -> Self {
        Self {
            egui_glium: EguiGlium::new(ViewportId::ROOT, display, window, event_loop),
            model_selection: ModelSelection::Cylinder,
            shading_mode: ShadingMode::Phong,
            lsystem_config: LSystemConfig::default(),
            interpolation_color_low: [0.28, 0.14, 0.01],
            interpolation_color_high: [0.08, 0.2, 0.01],
        }
    }

    pub fn get_lsystem_config(&self) -> &LSystemConfig {
        &self.lsystem_config
    }

    pub fn get_model_selection(&self) -> &ModelSelection {
        &self.model_selection
    }

    pub fn get_shading_mode(&self) -> &ShadingMode {
        &self.shading_mode
    }

    pub fn get_interpolation_colors(&self) -> ([f32; 3], [f32; 3]) {
        (self.interpolation_color_low, self.interpolation_color_high)
    }

    pub fn handle_event(&mut self, event: &WindowEvent, window: &Window) {
        let _ = self.egui_glium.on_event(window, event);
    }

    fn ui_control_panel(
        model_selection: &mut ModelSelection,
        shading_mode: &mut ShadingMode,
        ui: &mut Ui,
    ) {
        egui::ComboBox::from_label("Selected Model")
            .selected_text(format!("{model_selection:?}"))
            .show_ui(ui, |ui| {
                ui.selectable_value(model_selection, ModelSelection::Monkey, "Monkey");
                ui.selectable_value(model_selection, ModelSelection::Cylinder, "Cylinder");
            });

        ui.label("Shading Mode:");
        ui.radio_value(shading_mode, ShadingMode::Flat, "Flat");
        ui.radio_value(shading_mode, ShadingMode::Gouraud, "Gouraud");
        ui.radio_value(shading_mode, ShadingMode::Phong, "Phong");
    }

    // TODO make editable
    fn ui_lsystem_config(lsystem_config: &mut LSystemConfig, ui: &mut Ui) {
        ui.label("LSystem Config:");
        ui.add(
            egui::Slider::new(&mut lsystem_config.n_iterations, 0..=6).text("Number of Iterations"),
        );
        ui.add(egui::Slider::new(&mut lsystem_config.angle, 0.0..=45.0).text("Angle"));
        ui.label(format!("{:?}", lsystem_config.axiom));
        ui.label("Production Rules:");
        for (i, (symbol, replacement)) in lsystem_config.production_rules.iter().enumerate() {
            ui.horizontal(|ui| {
                ui.label(format!("{i}: {symbol} -> {replacement}"));
            });
        }
    }

    fn ui_color_panel(
        interpolation_color_low: &mut [f32; 3],
        interpolation_color_high: &mut [f32; 3],
        ui: &mut Ui,
    ) {
        ui.label("Color Interpolation:");
        ui.label("Color - low");
        ui.color_edit_button_rgb(interpolation_color_low);
        ui.label("Color - high");
        ui.color_edit_button_rgb(interpolation_color_high);
    }

    pub fn draw(&mut self, window: &Window, display: &Display<WindowSurface>, frame: &mut Frame) {
        let model_selection = &mut self.model_selection;
        let shading_mode = &mut self.shading_mode;
        let lsystem_config = &mut self.lsystem_config;
        let color_low = &mut self.interpolation_color_low;
        let color_high = &mut self.interpolation_color_high;

        self.egui_glium.run(window, |ctx| {
            egui::Window::new("Control panel").show(ctx, |ui| {
                GuiController::ui_control_panel(model_selection, shading_mode, ui);
                ui.separator();
                GuiController::ui_lsystem_config(lsystem_config, ui);
                ui.separator();
                GuiController::ui_color_panel(color_low, color_high, ui);
            });

        });
        self.egui_glium.paint(display, frame);
    }
}
