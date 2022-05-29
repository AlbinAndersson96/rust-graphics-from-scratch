mod win32;

pub fn create_window() -> Result<(), String> {
    if cfg!(target_os = "windows") {
        win32::new_window().expect("");
        Ok(())
    } else {
        return Err("Unsupported OS.".to_owned());
    }
}
