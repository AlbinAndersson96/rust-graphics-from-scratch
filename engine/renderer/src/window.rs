mod win32;

use std::{thread::JoinHandle, error::Error};

pub fn create_window() -> Result<JoinHandle<()>, Box<dyn Error>> {
    if cfg!(target_os = "windows") {
        let window_handle = std::thread::spawn(move || {
            let window_hwnd = win32::new_window().expect("");
            win32::show_window_start_event_loop(window_hwnd);
        });
        Ok(window_handle)
    } else {
        Err("Unsupported OS.")?
    }
}
