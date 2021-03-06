pub mod renderer;
pub mod input;
pub mod utils;

pub fn start() {
    let window_title = "TestWindow".to_owned();
    let window = renderer::create_window(800, 800, window_title).unwrap();
    window.join().unwrap();
}