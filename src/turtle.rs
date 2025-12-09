use glm::Mat4;

pub fn interpret_turtle_commands(lsystem: &str, angle: f32) -> Vec<Mat4> {
    let commands = parse_input(lsystem);

    let mut transformations: Vec<Mat4> = Vec::new();
    let mut state_stack: Vec<TurtleState> = Vec::new();

    let mut current_state = TurtleState {
        position: glm::vec3(0.0, 0.0, 0.0),
        direction: glm::vec3(0.0, 1.0, 0.0), // pointing up (Y+)
    };

    for command in &commands {
        match command {
            TurtleCommand::MoveForward => {
                let new_position = current_state.position + current_state.direction;

                // Translation to midpoint
                let translation = glm::translation(&current_state.position);

                // Rotation to align +Y with direction
                let up = glm::vec3(0.0, 1.0, 0.0);
                let axis = glm::cross(&up, &current_state.direction);
                let angle = up.dot(&current_state.direction).acos();
                let rotation = glm::rotation(angle, &glm::normalize(&axis));

                transformations.push(translation * rotation);
                current_state.position = new_position;
            }
            TurtleCommand::RotateLeft => {
                current_state =
                    handle_rotation(get_rotation_matrix(Axis::Z, angle), &current_state);
            }
            TurtleCommand::RotateRight => {
                current_state =
                    handle_rotation(get_rotation_matrix(Axis::Z, -angle), &current_state);
            }
            TurtleCommand::PitchUp => {
                current_state =
                    handle_rotation(get_rotation_matrix(Axis::X, angle), &current_state);
            }
            TurtleCommand::PitchDown => {
                current_state =
                    handle_rotation(get_rotation_matrix(Axis::X, -angle), &current_state);
            }
            TurtleCommand::RollLeft => {
                current_state =
                    handle_rotation(get_rotation_matrix(Axis::Y, angle), &current_state);
            }
            TurtleCommand::RollRight => {
                current_state =
                    handle_rotation(get_rotation_matrix(Axis::Y, -angle), &current_state);
            }
            TurtleCommand::PushState => {
                state_stack.push(current_state.clone());
            }
            TurtleCommand::PopState => {
                if let Some(state) = state_stack.pop() {
                    current_state = state;
                }
            }
        }
    }

    transformations
}

fn parse_input(input: &str) -> Vec<TurtleCommand> {
    input
        .chars()
        .filter_map(|c| TurtleCommand::try_from(c).ok())
        .collect()
}

fn handle_rotation(rot_matrix: Mat4, current_state: &TurtleState) -> TurtleState {
    let new_direction = (rot_matrix
        * glm::vec4(
            current_state.direction.x,
            current_state.direction.y,
            current_state.direction.z,
            0.0,
        ))
    .xyz();
    TurtleState {
        position: current_state.position,
        direction: glm::normalize(&new_direction),
    }
}

fn get_rotation_matrix(axis: Axis, angles: f32) -> Mat4 {
    let rotation_axis = match axis {
        Axis::X => glm::vec3(1.0, 0.0, 0.0),
        Axis::Y => glm::vec3(0.0, 1.0, 0.0),
        Axis::Z => glm::vec3(0.0, 0.0, 1.0),
    };
    glm::rotation(angles.to_radians(), &rotation_axis)
}

#[derive(Clone, Debug)]
struct TurtleState {
    position: glm::Vec3,
    direction: glm::Vec3,
}

enum TurtleCommand {
    MoveForward,
    RotateLeft,
    RotateRight,
    PitchUp,
    PitchDown,
    RollLeft,
    RollRight,
    PushState,
    PopState,
}

impl TryFrom<char> for TurtleCommand {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'F' => Ok(TurtleCommand::MoveForward),
            '+' => Ok(TurtleCommand::RotateLeft),
            '-' => Ok(TurtleCommand::RotateRight),
            '&' => Ok(TurtleCommand::PitchDown),
            '^' => Ok(TurtleCommand::PitchUp),
            '\\' => Ok(TurtleCommand::RollLeft),
            '/' => Ok(TurtleCommand::RollRight),
            '[' => Ok(TurtleCommand::PushState),
            ']' => Ok(TurtleCommand::PopState),
            _ => Err(format!("Ignoring non-drawing symbol: {value}")),
        }
    }
}

#[derive(Copy, Clone, Debug)]
enum Axis {
    X,
    Y,
    Z,
}
