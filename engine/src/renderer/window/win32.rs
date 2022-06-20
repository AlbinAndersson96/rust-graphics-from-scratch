#[cfg(windows)]

use winapi::um::winuser::{ CreateWindowExW, RegisterClassExW, PostQuitMessage, DefWindowProcW, ShowWindow, TranslateMessage, DispatchMessageA, PeekMessageA, LoadCursorW, LoadIconW,
    WNDCLASSEXW, MSG, 
    WS_OVERLAPPED, WS_MINIMIZEBOX, WS_SYSMENU, WM_DESTROY, SW_SHOW, PM_REMOVE, CS_OWNDC, CS_HREDRAW, CS_VREDRAW, IDI_APPLICATION, IDC_ARROW, COLOR_WINDOWFRAME };
use winapi::shared::windef::{ HBRUSH, HWND, HMENU };
use winapi::shared::minwindef::{ UINT, WPARAM, LPARAM, LRESULT, HINSTANCE };
use winapi::um::libloaderapi::GetModuleHandleW;

use std::ptr::null_mut;
use std::error::Error;

use crate::utils::utf16::to_utf16_vec;

static mut IS_WINDOW_CLOSED: bool = false;

pub fn get_window(window_width :u16, window_height :u16, window_title :String) -> Result<HWND, Box<dyn Error>>{
    let window_class_name = create_window_class()?;
    let window_hwnd = create_window(window_class_name, window_width, window_height, window_title)?;

    Ok(window_hwnd)
}

pub fn show_window_start_event_loop(window_hwnd: HWND) {
    unsafe {
        ShowWindow(window_hwnd, SW_SHOW);

        let mut msg: MSG = std::mem::zeroed();

        while !IS_WINDOW_CLOSED {
            if PeekMessageA(&mut msg, window_hwnd, 0, 0, PM_REMOVE) > 0 {
                TranslateMessage(&msg);
                DispatchMessageA(&msg);
            }
        }
    }
}

fn create_window(window_class_name: Vec<u16>, window_width :u16, window_height :u16, window_title :String) -> Result<HWND, Box<dyn Error>>{
    unsafe {
        let h_wnd_window = CreateWindowExW(
            0,
            window_class_name.as_ptr(),
            to_utf16_vec!(&window_title).as_ptr(),
            WS_OVERLAPPED | WS_MINIMIZEBOX | WS_SYSMENU,
            0,
            0,
            window_width.into(),
            window_height.into(),
            0 as HWND,
            0 as HMENU,
            0 as HINSTANCE,
            std::ptr::null_mut(),
        );

        if h_wnd_window == (0 as HWND)  {
            Err(format!("Failed to create window"))?;
        }

        Ok(h_wnd_window)
    }
}

fn create_window_class() -> Result<Vec<u16>, Box<dyn Error>> {    
    unsafe {
        let mut window_class_name: Vec<u16> = "WindowClass".encode_utf16().collect();
        window_class_name.push(0);

        let hinstance = GetModuleHandleW(null_mut());

        let window_class = WNDCLASSEXW {
            cbSize: std::mem::size_of::<WNDCLASSEXW>() as u32,
            style: CS_OWNDC | CS_HREDRAW | CS_VREDRAW,
            lpfnWndProc: Some(window_proc),
            cbClsExtra: 0,
            cbWndExtra: 0,
            hInstance: hinstance,
            hIcon: LoadIconW(null_mut(), IDI_APPLICATION),
            hCursor: LoadCursorW(null_mut(), IDC_ARROW),
            hbrBackground: COLOR_WINDOWFRAME as HBRUSH,
            lpszMenuName: null_mut(),
            lpszClassName: window_class_name.as_ptr(),
            hIconSm: LoadIconW(null_mut(), IDI_APPLICATION),
        };

        let error_code = RegisterClassExW(&window_class);

        if error_code == 0 {
            Err(format!("Failed to create window class, error code: {error_code}"))?;
        }

        Ok(window_class_name)
    }
}

unsafe extern "system" fn window_proc(h_wnd: HWND, msg: UINT, w_param: WPARAM, l_param: LPARAM) -> LRESULT {
    if msg == WM_DESTROY {
        IS_WINDOW_CLOSED = true;

        PostQuitMessage(0);
    }

    DefWindowProcW(h_wnd, msg, w_param, l_param)
}
