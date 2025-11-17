use crate::app::AppInteractionMode;
use crate::gui::GuiController;
use crate::shaders::make_shader_program;
use glium::glutin::surface::WindowSurface;
use glium::{
    Depth, DepthTest, Display, DrawParameters, Frame, Program, Surface, implement_vertex, uniform,
};
use glm::{Mat3, Mat4};
use tobj::Model;
use winit::event_loop::ActiveEventLoop;
use winit::window::Window;
use crate::lsystem::LSystem;
use crate::turtle::TurtleInterpreter;

pub struct Renderer {
    window: Window,
    display: Display<WindowSurface>,
    program: Program,
    pub gui: GuiController,
}

impl Renderer {
    pub fn new(
        window: Window,
        display: Display<WindowSurface>,
        event_loop: &ActiveEventLoop,
    ) -> Self {
        let program = make_shader_program(&display).expect("Failed to create shader program");
        let gui = GuiController::new(&display, &window, event_loop);

        Renderer {
            window,
            display,
            program,
            gui,
        }
    }

    pub fn request_redraw(&self) {
        self.window.request_redraw();
    }

    pub fn resize(&self, new_size: (u32, u32)) {
        self.display.resize(new_size);
    }

    pub fn handle_gui_event(&mut self, event: &winit::event::WindowEvent) {
        self.gui.handle_event(event, &self.window);
    }

    pub fn handle_interaction_mode_change(&mut self, mode: &AppInteractionMode) {
        match mode {
            AppInteractionMode::CameraControl => {
                if let Err(e) = self
                    .window
                    .set_cursor_grab(winit::window::CursorGrabMode::Confined)
                {
                    log::warn!("Could not grab cursor: {e:?}");
                }
                self.window.set_cursor_visible(false);
            }
            AppInteractionMode::GuiInteraction => {
                if let Err(e) = self
                    .window
                    .set_cursor_grab(winit::window::CursorGrabMode::None)
                {
                    log::warn!("Could not release cursor: {e:?}");
                }
                self.window.set_cursor_visible(true);
            }
        }
    }

    pub fn render_scene(
        &mut self,
        base: &Model,
        interaction_mode: &AppInteractionMode,
        view_matrix: [[f32; 4]; 4],
        projection_matrix: [[f32; 4]; 4],
    ) {
        let mut frame = self.display.draw();
        frame.clear_color_and_depth((0.0, 0.0, 0.0, 1.0), 1.0);

        let axiom = "F";
        let mut production_rules = std::collections::HashMap::new();
        production_rules.insert('F', "F+F--F+F".to_string());
        let lsystem = LSystem::new(axiom, production_rules);
        let lsystem_string = lsystem.generate(3);
        let turtle = TurtleInterpreter::new();
        let transformations = turtle.interpret(&lsystem_string, 15.0);


        for model_matrix in transformations.into_iter() {
            self.draw_model(
                &mut frame,
                base,
                model_matrix.into(),
                view_matrix,
                projection_matrix,
            );
        }

        if *interaction_mode == AppInteractionMode::GuiInteraction {
            self.gui.draw_ui(&self.window);
            self.gui.egui_glium.paint(&self.display, &mut frame);
        }

        frame.finish().expect("Failed to destroy frame");
    }

    pub fn draw_model(
        &mut self,
        frame: &mut Frame,
        model: &Model,
        model_matrix: [[f32; 4]; 4],
        view_matrix: [[f32; 4]; 4],
        projection_matrix: [[f32; 4]; 4],
    ) {
        let (vertices, indices) = Self::model_to_vertices_and_indices(model);
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
                &glium::VertexBuffer::new(&self.display, &vertices).unwrap(),
                &glium::IndexBuffer::new(
                    &self.display,
                    glium::index::PrimitiveType::TrianglesList,
                    &indices,
                )
                    .unwrap(),
                &self.program,
                &uniform! {model: model_matrix, view: view_matrix, projection: projection_matrix, normal_matrix: normal_matrix},
                &params,
            )
            .expect("Failed to draw frame");
    }

    fn model_to_vertices_and_indices(model: &Model) -> (Vec<Vertex>, Vec<u32>) {
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

        (vertices, mesh.indices.clone())
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
