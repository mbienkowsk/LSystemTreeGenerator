use std::collections::{HashMap, HashSet};

use glium::backend::glutin::SimpleWindowBuilder;
use winit::{
    application::ApplicationHandler,
    event::{DeviceId, ElementState, KeyEvent, WindowEvent},
    event_loop::ActiveEventLoop,
    keyboard::{KeyCode, PhysicalKey},
    window::WindowId,
};

const DELTA_TIME: f32 = 0.1;

use crate::common::ModelSelection;
use crate::gui::LSystemConfig;
use crate::lsystem::LSystem;
use crate::model_loader::{load_floor, load_model};
use crate::scene::Scene;
use crate::turtle::TurtleInterpreter;
use crate::{
    camera::{FlyCamera, MovementDirection},
    renderer::Renderer,
};

#[derive(Default, Debug, PartialEq)]
pub enum AppInteractionMode {
    #[default]
    CameraControl,
    GuiInteraction,
}

#[derive(Default)]
pub struct App {
    renderer: Option<Renderer>,
    camera: Option<FlyCamera>,
    pressed_keys: HashSet<KeyCode>,
    interaction_mode: AppInteractionMode,
    lsystem_config: Option<LSystemConfig>,
    model_selection: Option<ModelSelection>,
    scene: Option<Scene>,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let (window, display) = SimpleWindowBuilder::new()
            .with_title("L-System generator")
            .build(event_loop);

        self.renderer = Some(Renderer::new(window, display, event_loop));
        self.camera = Some(FlyCamera::new(
            glm::vec3(0.0, 1.0, 5.0),
            self.renderer.as_ref().unwrap().get_aspect_ratio(),
        ));
        self.scene = Some(Scene::new(
            load_floor(),
            load_model(ModelSelection::default()),
            Vec::new(),
            3.0,
            [10.0, 10.0, 10.0],
        ));

        self.renderer
            .as_mut()
            .unwrap()
            .handle_interaction_mode_change(&self.interaction_mode);

        self.lsystem_config = Some(self.get_current_lsystem_config().clone());
        self.calculate_transformations();
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        if let Some(renderer) = self.renderer.as_mut() {
            renderer.handle_gui_event(&event);
        }

        match event {
            WindowEvent::Resized(window_size) => {
                if let Some(renderer) = &self.renderer {
                    renderer.resize(window_size.into());
                }
                if let Some(camera) = &mut self.camera {
                    camera.update_aspect_ratio(window_size.into());
                }
            }
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::RedrawRequested => {
                if self.renderer.is_none() {
                    return;
                }

                self.update_fractal();

                self.render_scene();
                self.handle_movement();
            }
            WindowEvent::KeyboardInput { event, .. } => {
                self.handle_key_event(&event);
            }
            _ => {}
        }
    }

    fn device_event(
        &mut self,
        _event_loop: &ActiveEventLoop,
        _device_id: DeviceId,
        event: winit::event::DeviceEvent,
    ) {
        if let winit::event::DeviceEvent::MouseMotion { delta } = event {
            self.handle_mouse_movement(delta);
        }
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        if let Some(ref renderer) = self.renderer {
            renderer.request_redraw();
        }
    }
}

impl App {
    #[allow(clippy::cast_possible_truncation)]
    fn handle_mouse_movement(&mut self, delta: (f64, f64)) {
        if self.interaction_mode != AppInteractionMode::CameraControl {
            return;
        }

        if let Some(camera) = self.camera.as_mut() {
            let (delta_x, delta_y) = delta;
            camera.handle_mouse_movement(delta_x as f32, delta_y as f32);
        }
    }

    fn handle_key_event(&mut self, event: &KeyEvent) {
        match (event.state, event.physical_key) {
            (ElementState::Pressed, PhysicalKey::Code(KeyCode::Escape)) => {
                self.toggle_interaction_mode();
            }
            (ElementState::Pressed, PhysicalKey::Code(code)) => {
                self.pressed_keys.insert(code);
            }
            (ElementState::Released, PhysicalKey::Code(code)) => {
                self.pressed_keys.remove(&code);
            }
            _ => {}
        }
    }

    fn handle_movement(&mut self) {
        if self.interaction_mode != AppInteractionMode::CameraControl {
            return;
        }

        #[rustfmt::skip]
        let bindings: HashMap<Vec<KeyCode>, MovementDirection> = HashMap::from([
            (vec![KeyCode::KeyW, KeyCode::KeyK], MovementDirection::Forward),
            (vec![KeyCode::KeyS, KeyCode::KeyJ], MovementDirection::Backward),
            (vec![KeyCode::KeyA, KeyCode::KeyH], MovementDirection::Left),
            (vec![KeyCode::KeyD, KeyCode::KeyL], MovementDirection::Right),
            (vec![KeyCode::Space], MovementDirection::Up),
            (vec![KeyCode::KeyZ], MovementDirection::Down),
        ]);

        let camera = self.camera.as_mut().unwrap();
        for (key, value) in &bindings {
            if key.iter().any(|k| self.pressed_keys.contains(k)) {
                camera.handle_movement(value, DELTA_TIME);
            }
        }
    }

    fn toggle_interaction_mode(&mut self) {
        match self.interaction_mode {
            AppInteractionMode::CameraControl => {
                self.interaction_mode = AppInteractionMode::GuiInteraction;
            }
            AppInteractionMode::GuiInteraction => {
                self.interaction_mode = AppInteractionMode::CameraControl;
            }
        }
        self.renderer
            .as_mut()
            .unwrap()
            .handle_interaction_mode_change(&self.interaction_mode);
    }

    fn render_scene(&mut self) {
        let renderer = self.renderer.as_mut().unwrap();
        let camera = self.camera.as_ref().unwrap();

        renderer.render_scene(
            self.scene.as_ref().unwrap(),
            &self.interaction_mode,
            &camera.view_parameters(),
        );
    }

    fn update_fractal(&mut self) {
        let Some(renderer) = &self.renderer else {
            return;
        };

        let gui = renderer.get_gui_controller();
        let config = gui.get_lsystem_config();
        let model_selection = gui.get_model_selection();

        let config_changed = self.lsystem_config.as_ref() != Some(config);
        let model_changed = self.model_selection.as_ref() != Some(model_selection);

        if !config_changed && !model_changed {
            return;
        }

        log::info!(
            "Updating fractal - config_changed: {config_changed}, model_changed: {model_changed}"
        );

        if config_changed {
            log::info!("L-System config changed to {config:?}");
            self.lsystem_config = Some(config.clone());
        }

        if model_changed {
            log::info!("Model selection changed to {model_selection:?}");
            self.model_selection = Some(*model_selection);

            let new_base = load_model(*model_selection);

            if let Some(scene) = &mut self.scene {
                scene.set_fractal_base(new_base);
            }
        }

        let production_rules: HashMap<char, String> =
            config.production_rules.iter().cloned().collect();
        let lsystem = LSystem::new(&config.axiom, production_rules);
        let generated = lsystem.generate(config.n_iterations);
        let transformations = TurtleInterpreter::interpret(&generated, config.angle);

        if let Some(scene) = &mut self.scene {
            scene.update_transformations(transformations, config.fractal_height);
        }

        renderer.request_redraw();
    }

    fn calculate_transformations(&mut self) {
        let lsystem_config = self.get_current_lsystem_config();
        let target_height = lsystem_config.fractal_height;
        let production_rules: HashMap<char, String> =
            lsystem_config.production_rules.iter().cloned().collect();
        let lsystem = LSystem::new(&lsystem_config.axiom, production_rules);
        let generated_string = lsystem.generate(lsystem_config.n_iterations);
        let transformations = TurtleInterpreter::interpret(&generated_string, lsystem_config.angle);
        self.scene
            .as_mut()
            .unwrap()
            .update_transformations(transformations, target_height);
    }

    fn get_current_lsystem_config(&self) -> &LSystemConfig {
        self.renderer
            .as_ref()
            .unwrap()
            .get_gui_controller()
            .get_lsystem_config()
    }
}
