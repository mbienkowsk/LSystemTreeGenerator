use crate::app::AppInteractionMode;
use crate::gui::GuiController;
use crate::scene::Scene;
use crate::shaders::make_shader_program;

use glium::glutin::surface::WindowSurface;
use glium::{
    Depth, DepthTest, Display, DrawParameters, Frame, Program, Surface, implement_vertex, uniform,
};

use crate::camera::ViewParameters;
use glm::{Mat4, Vec3};
use tobj::Model;
use winit::event_loop::ActiveEventLoop;
use winit::window::Window;

pub struct Renderer {
    window: Window,
    display: Display<WindowSurface>,
    program: Program,
    gui: GuiController,
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

    pub fn get_gui_controller(&self) -> &GuiController {
        &self.gui
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
        scene: &Scene,
        interaction_mode: &AppInteractionMode,
        view_parameters: ViewParameters,
    ) {
        let mut frame = self.display.draw();
        frame.clear_color_and_depth((0.1, 0.1, 0.1, 1.0), 1.0);

        let shading_mode = i32::from(*self.gui.get_shading_mode());

        if !scene.transformations().is_empty() {
            let instance_data: Vec<InstanceData> = scene
                .transformations()
                .iter()
                .map(|&matrix| InstanceData::from_matrix(matrix))
                .collect();

            self.draw_model_instanced(
                &mut frame,
                scene.fractal_base(),
                &instance_data,
                &view_parameters,
                *scene.light_position(),
                shading_mode,
            );
        }

        // There is overhead in using instanced rendering for a single instance
        // But it is simpler this way
        let scale_matrix = glm::scale(&Mat4::identity(), &Vec3::new(10.0, 1.0, 10.0));
        let floor_instance = vec![InstanceData::from_matrix(scale_matrix)];
        self.draw_model_instanced(
            &mut frame,
            scene.floor(),
            &floor_instance,
            &view_parameters,
            *scene.light_position(),
            shading_mode,
        );

        if *interaction_mode == AppInteractionMode::GuiInteraction {
            self.gui.draw(&self.window, &self.display, &mut frame);
        }

        frame.finish().expect("Failed to destroy frame");
    }

    pub fn draw_model_instanced(
        &mut self,
        frame: &mut Frame,
        model: &Model,
        instance_data: &[InstanceData],
        view_parameters: &ViewParameters,
        light_pos: [f32; 3],
        shading_mode: i32,
    ) {
        let (vertices, indices) = Self::model_to_vertices_and_indices(model);

        let vertex_buffer = &glium::VertexBuffer::new(&self.display, &vertices).unwrap();
        let instance_buffer = glium::VertexBuffer::new(&self.display, instance_data).unwrap();
        let index_buffer = &glium::IndexBuffer::new(
            &self.display,
            glium::index::PrimitiveType::TrianglesList,
            &indices,
        )
        .unwrap();

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
                (vertex_buffer, instance_buffer.per_instance().unwrap()),
                index_buffer,
                &self.program,
                &uniform! {
                    view: view_parameters.view_matrix,
                    projection: view_parameters.projection_matrix,
                    u_light_pos: light_pos,
                    u_view_pos: view_parameters.camera_position,
                    u_shading_mode: shading_mode
                },
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

#[derive(Copy, Clone)]
#[allow(clippy::struct_field_names)]
pub struct InstanceData {
    model_matrix_0: [f32; 4],
    model_matrix_1: [f32; 4],
    model_matrix_2: [f32; 4],
    model_matrix_3: [f32; 4],
}

implement_vertex!(
    InstanceData,
    model_matrix_0,
    model_matrix_1,
    model_matrix_2,
    model_matrix_3
);

impl InstanceData {
    fn from_matrix(matrix: Mat4) -> Self {
        let m: [[f32; 4]; 4] = matrix.into();
        Self {
            model_matrix_0: m[0],
            model_matrix_1: m[1],
            model_matrix_2: m[2],
            model_matrix_3: m[3],
        }
    }
}
