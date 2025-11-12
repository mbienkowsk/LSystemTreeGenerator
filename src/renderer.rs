use crate::shaders::{FRAGMENT_SHADER_SRC, VERTEX_SHADER_SRC};
use glium::glutin::surface::WindowSurface;
use glium::{
    Depth, DepthTest, Display, DrawParameters, Program, Surface, implement_vertex, uniform,
};
use glm::Mat3;
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

    pub fn draw(
        &self,
        vertices: &[Vertex],
        indices: &[u16],
        model_matrix: [[f32; 4]; 4],
        view_matrix: [[f32; 4]; 4],
        projection_matrix: [[f32; 4]; 4],
    ) {
        let mut frame = self.display.draw();
        frame.clear_color_and_depth((0.0, 0.0, 0.0, 1.0), 1.0);
        let program =
            Program::from_source(&self.display, VERTEX_SHADER_SRC, FRAGMENT_SHADER_SRC, None);

        let model_mat3 = Mat3::from_fn(|r, c| model_matrix[r][c]);
        let normal_matrix: [[f32; 3]; 3] = glm::inverse_transpose(model_mat3).into();

        let params = DrawParameters {
            depth: Depth {
                test: DepthTest::IfLess,
                write: true,
                ..Depth::default()
            },
            ..DrawParameters::default()
        };

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
                &uniform! {model: model_matrix, view: view_matrix, projection: projection_matrix, normal_matrix: normal_matrix},
                &params,
            )
            .expect("Failed to draw frame");
        frame.finish().expect("Failed to destroy frame");
    }

    #[allow(clippy::too_many_lines)]
    pub fn draw_cube(
        &self,
        model_matrix: [[f32; 4]; 4],
        view_matrix: [[f32; 4]; 4],
        projection_matrix: [[f32; 4]; 4],
    ) {
        let vertices = vec![
            // Back face (z = -0.5)
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
            // Front face (z = 0.5)
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
            // Left face (x = -0.5)
            Vertex {
                position: [-0.5, -0.5, -0.5],
                normal: [-1.0, 0.0, 0.0],
            },
            Vertex {
                position: [-0.5, -0.5, 0.5],
                normal: [-1.0, 0.0, 0.0],
            },
            Vertex {
                position: [-0.5, 0.5, 0.5],
                normal: [-1.0, 0.0, 0.0],
            },
            Vertex {
                position: [-0.5, 0.5, -0.5],
                normal: [-1.0, 0.0, 0.0],
            },
            // Right face (x = 0.5)
            Vertex {
                position: [0.5, -0.5, -0.5],
                normal: [1.0, 0.0, 0.0],
            },
            Vertex {
                position: [0.5, -0.5, 0.5],
                normal: [1.0, 0.0, 0.0],
            },
            Vertex {
                position: [0.5, 0.5, 0.5],
                normal: [1.0, 0.0, 0.0],
            },
            Vertex {
                position: [0.5, 0.5, -0.5],
                normal: [1.0, 0.0, 0.0],
            },
            // Bottom face (y = -0.5)
            Vertex {
                position: [-0.5, -0.5, -0.5],
                normal: [0.0, -1.0, 0.0],
            },
            Vertex {
                position: [0.5, -0.5, -0.5],
                normal: [0.0, -1.0, 0.0],
            },
            Vertex {
                position: [0.5, -0.5, 0.5],
                normal: [0.0, -1.0, 0.0],
            },
            Vertex {
                position: [-0.5, -0.5, 0.5],
                normal: [0.0, -1.0, 0.0],
            },
            // Top face (y = 0.5)
            Vertex {
                position: [-0.5, 0.5, -0.5],
                normal: [0.0, 1.0, 0.0],
            },
            Vertex {
                position: [0.5, 0.5, -0.5],
                normal: [0.0, 1.0, 0.0],
            },
            Vertex {
                position: [0.5, 0.5, 0.5],
                normal: [0.0, 1.0, 0.0],
            },
            Vertex {
                position: [-0.5, 0.5, 0.5],
                normal: [0.0, 1.0, 0.0],
            },
        ];

        let indices: Vec<u16> = vec![
            0, 1, 2, 0, 2, 3, // Back
            4, 5, 6, 4, 6, 7, // Front
            8, 9, 10, 8, 10, 11, // Left
            12, 13, 14, 12, 14, 15, // Right
            16, 17, 18, 16, 18, 19, // Bottom
            20, 21, 22, 20, 22, 23, // Top
        ];

        self.draw(
            &vertices,
            &indices,
            model_matrix,
            view_matrix,
            projection_matrix,
        );
    }

    #[allow(clippy::cast_precision_loss)]
    pub fn get_aspect_ratio(&self) -> f32 {
        let size = self.window.inner_size();
        size.width as f32 / size.height as f32
    }
}

#[derive(Copy, Clone)]
pub struct Vertex {
    position: [f32; 3],
    normal: [f32; 3],
}
implement_vertex!(Vertex, position, normal);
