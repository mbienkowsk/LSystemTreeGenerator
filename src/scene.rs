use crate::model_loader::Model3D;
use glm::{Mat4, Vec4};
use tobj::Model;

pub struct Scene {
    floor: Model3D,
    fractal_base: Model3D,
    transformations: Vec<Vec<Mat4>>,
    target_height: f32,
    light_position: [f32; 3],
}

impl Scene {
    pub fn new(
        floor: Model3D,
        fractal_base: Model3D,
        transformations: Vec<Vec<Mat4>>,
        target_height: f32,
        light_position: [f32; 3],
    ) -> Self {
        Self {
            floor,
            fractal_base,
            transformations,
            target_height,
            light_position,
        }
    }

    pub fn floor(&self) -> &Model3D {
        &self.floor
    }

    pub fn fractal_base(&self) -> &Model3D {
        &self.fractal_base
    }

    pub fn transformations(&self) -> &Vec<Vec<Mat4>> {
        &self.transformations
    }

    pub fn target_height(&self) -> f32 {
        self.target_height
    }

    pub fn light_position(&self) -> &[f32; 3] {
        &self.light_position
    }

    pub fn update_transformations(&mut self, transformations: Vec<Vec<Mat4>>, target_height: f32) {
        let scaled_transformations = Self::scale_transformations_to_height(
            transformations,
            target_height,
            &self.fractal_base.geometry,
        );
        self.transformations = scaled_transformations;
        self.target_height = target_height;
    }

    pub fn set_fractal_base(&mut self, model: Model3D) {
        self.fractal_base = model;
    }

    /// Returns the maximum Y coordinate of the model's vertices
    fn model_max_local_y(model: &Model) -> f32 {
        model
            .mesh
            .positions
            .chunks(3)
            .map(|v| v[1])
            .fold(f32::NEG_INFINITY, f32::max)
    }

    fn fractal_total_height(base: &Model, transformations: &[Vec<Mat4>]) -> f32 {
        if transformations.is_empty() {
            return 0.0;
        }

        let model_height = Self::model_max_local_y(base);
        let up_vector = Vec4::new(0.0, model_height, 0.0, 1.0);

        transformations
            .iter()
            .flat_map(|mat_list| mat_list.iter().map(|mat| (mat * up_vector)[1]))
            .fold(f32::NEG_INFINITY, f32::max)
    }

    fn scale_transformations_to_height(
        transformations: Vec<Vec<Mat4>>,
        target_height: f32,
        base_model: &Model,
    ) -> Vec<Vec<Mat4>> {
        let current_height = Self::fractal_total_height(base_model, &transformations);
        if current_height == 0.0 {
            return transformations;
        }
        let scale_factor = target_height / current_height;
        let scale_matrix = glm::scale(
            &Mat4::identity(),
            &glm::vec3(scale_factor, scale_factor, scale_factor),
        );

        transformations
            .into_iter()
            .map(|mat_list| mat_list.into_iter().map(|mat| scale_matrix * mat).collect())
            .collect()
    }
}
