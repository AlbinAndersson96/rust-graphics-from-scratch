fn main() {
    let window_handle = renderer::init().expect("msg");
    
    window_handle.join().expect("dfad");
}