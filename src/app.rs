use crate::gui::TreeGenerationConfig;
use itertools::Itertools;
use rand::Rng;
use std::collections::{HashMap, HashSet};

use glium::backend::glutin::SimpleWindowBuilder;
use rand::distr::Uniform;
use tobj::Model;
use winit::{
    application::ApplicationHandler,
    event::{DeviceId, ElementState, KeyEvent, WindowEvent},
    event_loop::ActiveEventLoop,
    keyboard::{KeyCode, PhysicalKey},
    window::WindowId,
};

// TODO: this could probably be calculated based on time since last frame instead
const DELTA_TIME: f32 = 0.1;

use crate::gui::LSystemConfig;
use crate::lsystem::LSystem;
use crate::model_loader::{load_cylinder, load_floor, load_monkey};
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
    tree_generation_config: Option<TreeGenerationConfig>,
    base_models: Vec<Model>,
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
        self.base_models = vec![load_cylinder(), load_monkey()];
        self.scene = Some(Scene::new(
            load_floor(),
            self.base_models[0].clone(),
            Vec::new(),
            [10.0, 10.0, 10.0],
        ));

        self.renderer
            .as_mut()
            .unwrap()
            .handle_interaction_mode_change(&self.interaction_mode);

        self.lsystem_config = Some(self.get_current_lsystem_config().clone());
        self.tree_generation_config = Some(self.get_current_tree_generation_config().clone());
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

                // TODO: add check here, wrap lsystemconfig and treegenconfig
                if self.requires_transformation_recalculation() {
                    let new_lsystem_config = self.get_current_lsystem_config();
                    log::info!("L-System config changed to {new_lsystem_config:?}");
                    self.lsystem_config = Some(new_lsystem_config.clone());
                    let new_tree_generation_config = self.get_current_tree_generation_config();

                    log::info!("Tree generation config changed to {new_tree_generation_config:?}");
                    self.tree_generation_config = Some(new_tree_generation_config.clone());

                    self.calculate_transformations();
                }

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

        // TODO some reasonable base models for L-systems
        let model = match renderer.get_gui_controller().get_model_selection() {
            crate::gui::ModelSelection::Monkey => &self.base_models[1],
            crate::gui::ModelSelection::Cylinder => &self.base_models[0],
        };

        if model.name != self.scene.as_ref().unwrap().fractal_base().name {
            self.scene.as_mut().unwrap().set_fractal_base(model.clone());
        }

        let camera = self.camera.as_ref().unwrap();

        renderer.render_scene(
            self.scene.as_ref().unwrap(),
            &self.interaction_mode,
            &camera.view_parameters(),
        );
    }

    /// Generates N matrices with random transalations within the configured bounds
    #[allow(clippy::cast_precision_loss)]
    fn generate_displacement_matrices(&self) -> Vec<glm::Mat4> {
        let num_trees = self.get_current_tree_generation_config().get_num_trees();
        let mut rng = rand::rng();

        let tree_generation_config = self.get_current_tree_generation_config();

        let (xmin, xmax) = tree_generation_config.get_x_bounds();
        let (zmin, zmax) = tree_generation_config.get_z_bounds();

        (0..num_trees)
            .map(|_| {
                let x = rng.random_range(xmin..xmax) as f32;
                let z = rng.random_range(zmin..zmax) as f32;
                let y_rotation = rng.random_range(0.0..360.0_f32).to_radians();

                glm::translation(&glm::vec3(x, 0.0, z))
                    * glm::rotation(y_rotation, &glm::vec3(0.0, 1.0, 0.0))
            })
            .collect::<Vec<glm::Mat4>>()
    }

    fn calculate_transformations(&mut self) {
        let lsystem_config = self.get_current_lsystem_config();

        let production_rules: HashMap<char, String> =
            lsystem_config.production_rules.iter().cloned().collect();
        let lsystem = LSystem::new(&lsystem_config.axiom, production_rules);
        let generated_string = lsystem.generate(lsystem_config.n_iterations);
        let transformations = TurtleInterpreter::interpret(&generated_string, lsystem_config.angle);

        let displacement_matrices = self.generate_displacement_matrices();

        let final_transformations = displacement_matrices
            .iter()
            .flat_map(|displacement_matrix| {
                transformations
                    .iter()
                    .map(move |transformation| displacement_matrix * transformation)
            })
            .collect::<Vec<glm::Mat4>>();

        self.scene
            .as_mut()
            .unwrap()
            .update_transformations(final_transformations);
    }

    fn get_current_lsystem_config(&self) -> &LSystemConfig {
        self.renderer
            .as_ref()
            .unwrap()
            .get_gui_controller()
            .get_lsystem_config()
    }

    fn get_current_tree_generation_config(&self) -> &TreeGenerationConfig {
        self.renderer
            .as_ref()
            .unwrap()
            .get_gui_controller()
            .get_tree_generation_config()
    }

    fn requires_transformation_recalculation(&mut self) -> bool {
        let new_lsystem_config = self.get_current_lsystem_config();
        let new_tree_generation_config = self.get_current_tree_generation_config();
        new_lsystem_config != self.lsystem_config.as_ref().unwrap()
            || new_tree_generation_config != self.tree_generation_config.as_ref().unwrap()
    }
}
