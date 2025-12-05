use glm::{Mat4, Vec4};
use tobj::Model;

pub struct Scene {
    floor: Model,
    fractal_base: Model,
    transformations: Vec<Mat4>,
    light_position: [f32; 3],
}

impl Scene {
    pub fn new(
        floor: Model,
        fractal_base: Model,
        transformations: Vec<Mat4>,
        light_position: [f32; 3],
    ) -> Self {
        Self {
            floor,
            fractal_base,
            transformations,
            light_position,
        }
    }

    pub fn floor(&self) -> &Model {
        &self.floor
    }

    pub fn fractal_base(&self) -> &Model {
        &self.fractal_base
    }

    pub fn transformations(&self) -> &[Mat4] {
        &self.transformations
    }

    pub fn light_position(&self) -> &[f32; 3] {
        &self.light_position
    }

    pub fn update_transformations(&mut self, transformations: Vec<Mat4>) {
        self.transformations = transformations;
    }

    pub fn set_fractal_base(&mut self, model: Model) {
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

    pub fn fractal_total_height(&self) -> f32 {
        if self.transformations.is_empty() {
            return 0.0;
        }

        let model_height = Self::model_max_local_y(&self.fractal_base);
        let up_vector = Vec4::new(0.0, model_height, 0.0, 1.0);

        self.transformations
            .iter()
            .map(|mat| (mat * up_vector)[1])
            .fold(f32::NEG_INFINITY, f32::max)
    }
}
