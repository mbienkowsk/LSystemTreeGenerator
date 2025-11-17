// TODO rotations in 3D
// Probably will have to use quaternions for that

pub struct TurtleInterpreter {}

const ALLOWED_SYMBOLS: &[char] = &['F', '+', '-', '[', ']'];

struct TurtleState {
    position: glm::Vec3,
    direction: glm::Vec3,
}

impl TurtleInterpreter {
    pub fn new() -> Self {
        Self {}
    }

    fn validate_input(&self, input: &str) -> bool {
        input.chars().all(|c| ALLOWED_SYMBOLS.contains(&c))
    }

    pub fn interpret(&self, lsystem: &str, angle: f32) -> Vec<glm::Mat4> {
        if !self.validate_input(lsystem) {
            panic!("Input contains invalid symbols");
        }

        let mut transformations: Vec<glm::Mat4> = Vec::new();
        let mut state_stack: Vec<TurtleState> = Vec::new();

        let mut current_state = TurtleState {
            position: glm::vec3(0.0, 0.0, 0.0),
            direction: glm::vec3(0.0, 1.0, 0.0), // pointing up (Y+)
        };

        for symbol in lsystem.chars() {
            match symbol {
                'F' => {
                    // Move forward and create transformation
                    let new_position = current_state.position + current_state.direction;

                    // Translation to midpoint
                    let translation = glm::translation(&current_state.position);

                    // Rotation to align +Y with direction
                    let up = glm::vec3(0.0, 1.0, 0.0);
                    let axis = glm::cross(&up, &current_state.direction);
                    let rotation = if axis.magnitude() > 1e-6 {
                        let angle = up.dot(&current_state.direction).acos();
                        glm::rotation(angle, &glm::normalize(&axis))
                    } else {
                        glm::Mat4::identity()
                    };

                    transformations.push(translation * rotation);
                    current_state.position = new_position;
                }
                '+' => {
                    // Rotate left around Z-axis
                    let rotation = glm::rotation(angle.to_radians(), &glm::vec3(0.0, 0.0, 1.0));
                    current_state.direction = (rotation
                        * glm::vec4(
                            current_state.direction.x,
                            current_state.direction.y,
                            current_state.direction.z,
                            0.0,
                        ))
                    .xyz();
                    current_state.direction = glm::normalize(&current_state.direction);
                }
                '-' => {
                    // Rotate right around Z-axis
                    let rotation = glm::rotation(-angle.to_radians(), &glm::vec3(0.0, 0.0, 1.0));
                    current_state.direction = (rotation
                        * glm::vec4(
                            current_state.direction.x,
                            current_state.direction.y,
                            current_state.direction.z,
                            0.0,
                        ))
                    .xyz();
                    current_state.direction = glm::normalize(&current_state.direction);
                }
                '[' => {
                    // Push current state
                    state_stack.push(TurtleState {
                        position: current_state.position,
                        direction: current_state.direction,
                    });
                }
                ']' => {
                    // Pop previous state
                    if let Some(state) = state_stack.pop() {
                        current_state = state;
                    }
                }
                _ => {}
            }
        }

        transformations
    }
}
