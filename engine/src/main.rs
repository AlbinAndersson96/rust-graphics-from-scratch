fn main() {
    let window_title = "TestWindow";
    let window = renderer::create_window(800, 800, window_title).unwrap();
    window.join().unwrap();
}