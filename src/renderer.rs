use glium::glutin::surface::WindowSurface;
use glium::{Display, DrawParameters, Program, Surface, implement_vertex};
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

    pub fn draw(&self, vertices: &Vec<Vertex>, indices: &Vec<u16>) {
        let mut frame = self.display.draw();
        frame.clear_color(1.0, 0.0, 0.0, 1.0);
        let program = Program::from_source(
            &self.display,
            "
            #version 140
            in vec3 position;
            in vec3 normal;
            void main() {
                gl_Position = vec4(position, 1.0);
            }
            ",
            "
            #version 140
            out vec4 color;
            void main() {
                color = vec4(0.0, 1.0, 0.0, 1.0);
            }
            ",
            None,
        );
        frame
            .draw(
                &glium::VertexBuffer::new(&self.display, vertices).unwrap(),
                &glium::IndexBuffer::new(
                    &self.display,
                    glium::index::PrimitiveType::TrianglesList,
                    indices,
                )
                .unwrap(),
                &program.unwrap(),
                &glium::uniforms::EmptyUniforms,
                &DrawParameters::default(),
            )
            .expect("Failed to draw frame");
        frame.finish().expect("Failed to destroy frame");
    }

    pub fn draw_cube(&self) {
        let vertices = vec![
            Vertex {
                position: [-0.5, -0.5, -0.5],
                normal: [0.0, 0.0, -1.0],
            },
            Vertex {
                position: [0.5, -0.5, -0.5],
                normal: [0.0, 0.0, -1.0],
            },
            Vertex {
                position: [0.5, 0.5, -0.5],
                normal: [0.0, 0.0, -1.0],
            },
            Vertex {
                position: [-0.5, 0.5, -0.5],
                normal: [0.0, 0.0, -1.0],
            },
            Vertex {
                position: [-0.5, -0.5, 0.5],
                normal: [0.0, 0.0, 1.0],
            },
            Vertex {
                position: [0.5, -0.5, 0.5],
                normal: [0.0, 0.0, 1.0],
            },
            Vertex {
                position: [0.5, 0.5, 0.5],
                normal: [0.0, 0.0, 1.0],
            },
            Vertex {
                position: [-0.5, 0.5, 0.5],
                normal: [0.0, 0.0, 1.0],
            },
        ];

        let indices = vec![
            0u16, 1, 2, 2, 3, 0, 4u16, 5, 6, 6, 7, 4, 0u16, 4, 7, 7, 3, 0, 1u16, 5, 6, 6, 2, 1,
            3u16, 2, 6, 6, 7, 3, 0u16, 1, 5, 5, 4, 0,
        ];

        self.draw(&vertices, &indices);
    }
}

#[derive(Copy, Clone)]
pub struct Vertex {
    position: [f32; 3],
    normal: [f32; 3],
}
implement_vertex!(Vertex, position, normal);
