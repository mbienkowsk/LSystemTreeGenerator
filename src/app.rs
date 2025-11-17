use std::collections::{HashMap, HashSet};

use glium::backend::glutin::SimpleWindowBuilder;
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

use crate::gui::GuiController;
use crate::model_loader::{load_cone, load_floor, load_monkey};
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
    models: Vec<Model>,
    interaction_mode: AppInteractionMode,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let (window, display) = SimpleWindowBuilder::new()
            .with_title("L-System generator")
            .build(event_loop);

        let gui_renderer = GuiController::new(&display, &window, event_loop);
        self.renderer = Some(Renderer::new(window, display, gui_renderer));
        self.camera = Some(FlyCamera::new(
            glm::vec3(0.0, 0.0, 5.0),
            self.renderer.as_ref().unwrap().get_aspect_ratio(),
        ));
        self.models = vec![load_monkey(), load_cone(), load_floor()];

        self.renderer
            .as_mut()
            .unwrap()
            .handle_interaction_mode_change(&self.interaction_mode);
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
            renderer.requrest_redraw();
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

        let model = match renderer.gui.get_model_selection() {
            crate::gui::ModelSelection::Monkey => &self.models[0],
            crate::gui::ModelSelection::Cone => &self.models[1],
        };

        renderer.render_scene(
            std::slice::from_ref(model),
            &self.interaction_mode,
            self.camera.as_ref().unwrap().get_view_matrix(),
            self.camera.as_ref().unwrap().get_projection_matrix(),
        );
    }
}
