use glium::glutin::surface::WindowSurface;
use glium::{Display, Surface};
use winit::window::Window;

pub struct Renderer {
    window: Window,
    display: Display<WindowSurface>,
}

impl Renderer {
    pub fn new(window: Window, display: Display<WindowSurface>) -> Self {
        Renderer { window, display }
    }

    pub fn requrest_redraw(&self) {
        self.window.request_redraw();
    }

    pub fn resize(&self, new_size: (u32, u32)) {
        self.display.resize(new_size);
    }

    pub fn draw(&self) {
        let mut frame = self.display.draw();
        frame.clear_color(1.0, 0.0, 0.0, 1.0);
        frame.finish().expect("Failed to destroy frame");
    }
}
