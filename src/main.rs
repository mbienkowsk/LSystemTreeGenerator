use app::App;
use winit::event_loop::EventLoop;
extern crate nalgebra_glm as glm;

mod app;
mod camera;
mod renderer;
mod shaders;
mod model_loader;

fn main() {
    env_logger::init();
    let event_loop = EventLoop::new().unwrap();
    event_loop.run_app(&mut App::default()).unwrap();
}

#[cfg(test)]
mod tests {
    #[test]
    fn dummy_test() {
        assert_eq!(2 + 2, 4);
    }
}
