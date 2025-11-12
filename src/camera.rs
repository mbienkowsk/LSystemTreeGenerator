const WORLD_UP: glm::Vec3 = glm::Vec3::new(0.0, 1.0, 0.0);
const MIN_PITCH: f32 = -89.0;
const MAX_PITCH: f32 = 89.0;

pub struct FlyCamera {
    position: glm::Vec3,
    // direction camera is facing
    front: glm::Vec3,

    /// left-right rotation (mouse x) in degrees
    yaw: f32,
    /// up-down rotation (mouse y) in degrees
    pitch: f32,

    aspect_ratio: f32,
    /// zoom in degrees
    fovy: f32,

    /// objects closer than that will be clipped
    znear: f32,
    /// objects  further than that will be clipped
    zfar: f32,
    /// rate of position updates
    speed: f32,
    /// rate of yaw/pitch updates
    sensitivity: f32,
}

impl FlyCamera {
    pub fn new(position: glm::Vec3, aspect_ratio: f32) -> Self {
        Self {
            // i made these constants up, but who would want to parametrize all this
            position,
            front: glm::vec3(0.0, 0.0, -1.0),
            yaw: -90.0,
            pitch: 0.0,

            aspect_ratio,
            fovy: 45.0,
            znear: 0.1,
            zfar: 100.0,

            speed: 0.5,
            sensitivity: 0.2,
        }
    }

    pub fn get_view_matrix(&self) -> [[f32; 4]; 4] {
        let view_matrix = glm::look_at(&self.position, &(self.position + self.front), &WORLD_UP);
        view_matrix.into()
    }

    pub fn get_projection_matrix(&self) -> [[f32; 4]; 4] {
        let projection_matrix = glm::perspective_rh_zo(
            self.aspect_ratio,
            self.fovy.to_radians(),
            self.znear,
            self.zfar,
        );
        projection_matrix.into()
    }

    pub fn update_aspect_ratio(&mut self, size: (f32, f32)) {
        self.aspect_ratio = size.0 / size.1;
    }

    pub fn handle_movement(&mut self, direction: &MovementDirection, delta_time: f32) {
        log::info!("Movement: {direction:?} with delta_time={delta_time}",);
        let velocity = self.speed * delta_time;
        let right = glm::normalize(&glm::cross(&self.front, &WORLD_UP));
        match direction {
            MovementDirection::Forward => self.position += self.front * velocity,
            MovementDirection::Backward => self.position -= self.front * velocity,
            MovementDirection::Left => self.position -= right * velocity,
            MovementDirection::Right => self.position += right * velocity,
            MovementDirection::Up => self.position += WORLD_UP * velocity,
            MovementDirection::Down => self.position -= WORLD_UP * velocity,
        }
    }

    fn update_front(&mut self) {
        let front = glm::vec3(
            self.yaw.to_radians().cos() * self.pitch.to_radians().cos(),
            self.pitch.to_radians().sin(),
            self.yaw.to_radians().sin() * self.pitch.to_radians().cos(),
        );
        self.front = glm::normalize(&front);
    }

    fn update_pitch(&mut self, yoffset: f32) {
        self.pitch -= yoffset;

        match self.pitch.clamp(MIN_PITCH, MAX_PITCH) {
            MAX_PITCH => log::trace!("Pitch clamped to MAX_PITCH"),
            MIN_PITCH => log::trace!("Pitch clamped to MIN_PITCH"),
            _ => {}
        }
    }

    pub fn handle_mouse_movement(&mut self, xoffset: f32, yoffset: f32) {
        log::trace!("Mouse movement detected: xoffset={xoffset}, yoffset={yoffset}");
        let xoffset = xoffset * self.sensitivity;
        let yoffset = yoffset * self.sensitivity;
        self.yaw += xoffset;
        self.update_pitch(yoffset);
        self.update_front();
    }
}

#[derive(Debug)]
pub enum MovementDirection {
    Forward,
    Backward,
    Left,
    Right,
    Up,
    Down,
}
