use glm::Mat4;
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
}
