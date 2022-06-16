pub mod window;

use std::{thread::JoinHandle, error::Error};

pub fn create_window(window_width :u16, window_height :u16, window_title :String) -> Result<JoinHandle<()>, Box<dyn Error>> {
    let window = window::create_window(window_width, window_height, window_title)?;
    Ok(window)
}

