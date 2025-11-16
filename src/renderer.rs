use crate::shaders::make_shader_program;
use glium::glutin::surface::WindowSurface;
use glium::{
    Depth, DepthTest, Display, DrawParameters, Program, Surface, implement_vertex, uniform,
};
use glm::Mat3;
use tobj::Model;
use winit::window::Window;

pub struct Renderer {
    window: Window,
    display: Display<WindowSurface>,
    program: Program,
}

impl Renderer {
    pub fn new(window: Window, display: Display<WindowSurface>) -> Self {
        let program = make_shader_program(&display).expect("Failed to create shader program");

        Renderer {
            window,
            display,
            program,
        }
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
                &self.program,
                &uniform! {model: model_matrix, view: view_matrix, projection: projection_matrix, normal_matrix: normal_matrix},
                &params,
            )
            .expect("Failed to draw frame");
        frame.finish().expect("Failed to destroy frame");
    }

    pub fn draw_model(
        &self,
        model: &Model,
        model_matrix: [[f32; 4]; 4],
        view_matrix: [[f32; 4]; 4],
        projection_matrix: [[f32; 4]; 4],
    ) {
        let (vertices, indices) = Self::model_to_vertices_and_indices(model);
        self.draw(
            &vertices,
            &indices,
            model_matrix,
            view_matrix,
            projection_matrix,
        );
    }

    fn model_to_vertices_and_indices(model: &Model) -> (Vec<Vertex>, Vec<u16>) {
        let mesh = &model.mesh;
        let positions = &mesh.positions;
        let normals = &mesh.normals;

        assert_eq!(positions.len() % 3, 0);
        assert_eq!(positions.len(), normals.len());
        let n_vertices = positions.len() / 3;

        let vertices: Vec<Vertex> = (0..n_vertices)
            .map(|i| Vertex {
                position: [positions[i * 3], positions[i * 3 + 1], positions[i * 3 + 2]],
                normal: [normals[i * 3], normals[i * 3 + 1], normals[i * 3 + 2]],
            })
            .collect();

        let indices: Vec<u16> = mesh.indices.iter().map(|&i| i as u16).collect();

        (vertices, indices)
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
