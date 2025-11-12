use std::collections::{HashMap, HashSet};
use winit::window::CursorGrabMode;

use glium::backend::glutin::SimpleWindowBuilder;
use glm::Mat4;
use winit::{
    application::ApplicationHandler,
    event::{DeviceId, ElementState, KeyEvent, WindowEvent},
    event_loop::ActiveEventLoop,
    keyboard::{KeyCode, PhysicalKey},
    window::WindowId,
};

const DELTA_TIME: f32 = 0.1;

use crate::{
    camera::{FlyCamera, MovementDirection},
    renderer::Renderer,
};

#[derive(Default)]
pub struct App {
    renderer: Option<Renderer>,
    camera: Option<FlyCamera>,
    pressed_keys: HashSet<KeyCode>,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let (window, display) = SimpleWindowBuilder::new()
            .with_title("L-System generator")
            .build(event_loop);

        if let Err(e) = window.set_cursor_grab(CursorGrabMode::Confined) {
            log::warn!("Could not grab cursor: {e:?}");
        }
        window.set_cursor_visible(false);

        self.renderer = Some(Renderer::new(window, display));
        self.camera = Some(FlyCamera::new(
            glm::vec3(0.0, 0.0, 5.0),
            self.renderer.as_ref().unwrap().get_aspect_ratio(),
        ));
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        if let Some(ref renderer) = self.renderer {
            renderer.requrest_redraw();
        }
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
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

                let renderer = self.renderer.as_ref().unwrap();
                let model = Mat4::identity();
                renderer.draw_cube(
                    model.into(),
                    self.camera.as_ref().unwrap().get_view_matrix(),
                    self.camera.as_ref().unwrap().get_projection_matrix(),
                );

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
}

impl App {
    fn handle_mouse_movement(&mut self, delta: (f64, f64)) {
        if let Some(camera) = self.camera.as_mut() {
            let (delta_x, delta_y) = delta;
            camera.handle_mouse_movement(delta_x as f32, delta_y as f32);
        }
    }

    fn handle_key_event(&mut self, event: &KeyEvent) {
        match (event.state, event.physical_key) {
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
}
