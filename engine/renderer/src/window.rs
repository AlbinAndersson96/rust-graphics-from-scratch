mod win32;

use std::{thread::JoinHandle, error::Error};

pub fn create_window(window_width :u16, window_height :u16, window_title :&'static str) -> Result<JoinHandle<()>, Box<dyn Error>> {
    if cfg!(target_os = "windows") {
        let window_handle = std::thread::spawn(move || {
            let window_hwnd = win32::get_window(window_width, window_height, window_title).unwrap();
            win32::show_window_start_event_loop(window_hwnd);
        });
        Ok(window_handle)
    } else {
        Err("Unsupported OS.")?
    }
}
