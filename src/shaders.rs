use glium::glutin::surface::WindowSurface;
use glium::program::ProgramCreationError;
use glium::{Display, Program};

const VERTEX_SHADER_SRC: &str = include_str!("../assets/shaders/vertex.glsl");
const FRAGMENT_SHADER_SRC: &str = include_str!("../assets/shaders/fragment.glsl");

pub fn make_shader_program(
    display: &Display<WindowSurface>,
) -> Result<Program, ProgramCreationError> {
    Program::from_source(display, VERTEX_SHADER_SRC, FRAGMENT_SHADER_SRC, None)
}
