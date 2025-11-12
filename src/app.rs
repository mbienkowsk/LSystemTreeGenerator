use glium::backend::glutin::SimpleWindowBuilder;
use glm::Mat4;
use winit::{
    application::ApplicationHandler,
    event::{DeviceId, KeyEvent, WindowEvent},
    event_loop::ActiveEventLoop,
    keyboard::{KeyCode, PhysicalKey},
    window::WindowId,
};

const DELTA_TIME: f32 = 1.0;

use crate::{camera::FlyCamera, renderer::Renderer};

#[derive(Default)]
pub struct App {
    renderer: Option<Renderer>,
    camera: Option<FlyCamera>,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let (window, display) = SimpleWindowBuilder::new()
            .with_title("L-System generator")
            .build(event_loop);

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
                if let Some(renderer) = &self.renderer {
                    let model = Mat4::identity();
                    let model = glm::rotate_x(&model, 45.0f32.to_radians());
                    let model = glm::rotate_y(&model, 45.0f32.to_radians());

                    renderer.draw_cube(
                        model.into(),
                        self.camera.as_ref().unwrap().get_view_matrix(),
                        self.camera.as_ref().unwrap().get_projection_matrix(),
                    );
                }
            }
            WindowEvent::KeyboardInput { event, .. } => self.handle_keypress(&event),
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

    fn handle_keypress(&mut self, event: &KeyEvent) {
        let camera = self.camera.as_mut().unwrap();

        if let PhysicalKey::Code(key_code) = event.physical_key {
            match key_code {
                KeyCode::KeyW => camera.move_forward(DELTA_TIME),
                KeyCode::KeyS => camera.move_backward(DELTA_TIME),
                KeyCode::KeyA => camera.move_left(DELTA_TIME),
                KeyCode::KeyD => camera.move_right(DELTA_TIME),
                KeyCode::ArrowUp | KeyCode::KeyK => camera.move_up(DELTA_TIME),
                KeyCode::ArrowDown | KeyCode::KeyJ => camera.move_down(DELTA_TIME),
                _ => {}
            }
        }
    }
}
