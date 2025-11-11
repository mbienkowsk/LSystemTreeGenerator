use app::App;
use winit::event_loop::EventLoop;

mod app;
mod renderer;

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
