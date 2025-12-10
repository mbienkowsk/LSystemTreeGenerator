use crate::app::AppInteractionMode;
use crate::gui::GuiController;
use crate::scene::Scene;
use crate::shaders::make_shader_program;

use glium::glutin::surface::WindowSurface;
use glium::{
    Depth, DepthTest, Display, DrawParameters, Frame, Program, Surface, implement_vertex, uniform,
};

use crate::camera::ViewParameters;
use crate::model_loader::Model3D;
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
        view_parameters: &ViewParameters,
    ) {
        let mut frame = self.display.draw();
        frame.clear_color_and_depth((0.1, 0.1, 0.1, 1.0), 1.0);

        let shading_mode = i32::from(*self.gui.get_shading_mode());
        let (interpolation_color_low, interpolation_color_high) =
            self.gui.get_interpolation_colors();

        if !scene.transformations().is_empty() {
            let instance_data: Vec<InstanceData> = scene
                .transformations()
                .iter()
                .flat_map(|transform_list| {
                    transform_list
                        .iter()
                        .map(|matrix| InstanceData::from_matrix(*matrix))
                })
                .collect();

            self.draw_model_instanced(
                &mut frame,
                scene.fractal_base(),
                &instance_data,
                view_parameters,
                *scene.light_position(),
                shading_mode,
                scene.target_height(),
                interpolation_color_low,
                interpolation_color_high,
                ColorMode::Interpolated,
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
            view_parameters,
            *scene.light_position(),
            shading_mode,
            1.0,
            interpolation_color_low,
            interpolation_color_high,
            ColorMode::Material,
        );

        if *interaction_mode == AppInteractionMode::GuiInteraction {
            self.gui.draw(&self.window, &self.display, &mut frame);
        }

        frame.finish().expect("Failed to destroy frame");
    }

    #[allow(clippy::too_many_arguments)]
    pub fn draw_model_instanced(
        &mut self,
        frame: &mut Frame,
        model: &Model3D,
        instance_data: &[InstanceData],
        view_parameters: &ViewParameters,
        light_pos: [f32; 3],
        shading_mode: i32,
        total_fractal_height: f32,
        interpolation_color_low: [f32; 3],
        interpolation_color_high: [f32; 3],
        color_mode: ColorMode,
    ) {
        let (vertices, indices) = Self::model_to_vertices_and_indices(&model.geometry);

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

        let uniforms = &uniform! {
            view: view_parameters.view_matrix,
            projection: view_parameters.projection_matrix,
            u_light_pos: light_pos,
            u_view_pos: view_parameters.camera_position,
            u_shading_mode: shading_mode,
            u_interpolation_color_low: interpolation_color_low,
            u_interpolation_color_high: interpolation_color_high,
            u_total_height: total_fractal_height,
            u_color_mode: i32::from(color_mode),
            u_material_ambient: model.material.ambient.unwrap(),
            u_material_diffuse: model.material.diffuse.unwrap(),
            u_material_specular: model.material.specular.unwrap(),
        };

        frame
            .draw(
                (vertex_buffer, instance_buffer.per_instance().unwrap()),
                index_buffer,
                &self.program,
                uniforms,
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

    pub fn unset_requires_tree_regeneration(&mut self) {
        self.gui.unset_requires_tree_regeneration();
    }
}

#[derive(Copy, Clone)]
pub struct Vertex {
    position: [f32; 3],
    normal: [f32; 3],
}
implement_vertex!(Vertex, position, normal);

#[derive(Copy, Clone)]
pub struct InstanceData {
    model_matrix: [[f32; 4]; 4],
}

implement_vertex!(InstanceData, model_matrix);

impl InstanceData {
    fn from_matrix(matrix: Mat4) -> Self {
        Self {
            model_matrix: matrix.into(),
        }
    }
}

pub enum ColorMode {
    Material,
    Interpolated,
}

impl From<ColorMode> for i32 {
    fn from(mode: ColorMode) -> Self {
        match mode {
            ColorMode::Material => 0,
            ColorMode::Interpolated => 1,
        }
    }
}
