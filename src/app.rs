use glium::backend::glutin::SimpleWindowBuilder;
use glm::Mat4;
use winit::{
    application::ApplicationHandler, event::WindowEvent, event_loop::ActiveEventLoop,
    window::WindowId,
};

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
            _ => {}
        }
    }
}
