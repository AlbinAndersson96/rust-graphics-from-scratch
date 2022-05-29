mod window;

use std::{thread::JoinHandle, error::Error};

pub fn init() -> Result<JoinHandle<()>, Box<dyn Error>> {
    let window_handle = window::create_window()?;
    Ok(window_handle)
}