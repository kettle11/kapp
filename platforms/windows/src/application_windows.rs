use std::ptr::null_mut;
use winapi::shared::minwindef;
use winapi::shared::windef;
use winapi::um::libloaderapi;
use winapi::um::winuser;

use crate::utils_windows::*;
use minwindef::{FALSE, TRUE};

use crate::{Cursor, PlatformApplicationTrait, PlatformEventLoopTrait, WindowId, WindowParameters};

// These should be made into something safe.
pub static mut CURRENT_CURSOR: windef::HCURSOR = null_mut();
pub static mut WINDOWS_TO_REDRAW: Vec<WindowId> = Vec::new();

pub struct PlatformApplication {
    window_class_name: Vec<u16>,
    h_instance: minwindef::HINSTANCE,
}

impl PlatformApplicationTrait for PlatformApplication {
    type EventLoop = PlatformEventLoop;
    fn new() -> Self {
        unsafe {
            // Register the window class.
            let window_class_name = win32_string("windowing_rust");
            let h_instance = libloaderapi::GetModuleHandleW(null_mut());

            let window_class = winuser::WNDCLASSW {
                style: 0,
                lpfnWndProc: Some(super::event_loop_windows::window_callback),
                cbClsExtra: 0,
                cbWndExtra: 0,
                hInstance: h_instance,
                hIcon: null_mut(),
                hCursor: null_mut(), // This may not be what is desired. Potentially this makes it annoying to change the cursor later.
                hbrBackground: null_mut(),
                lpszMenuName: null_mut(),
                lpszClassName: window_class_name.as_ptr(),
            };

            CURRENT_CURSOR = winuser::LoadCursorW(null_mut(), winuser::IDC_ARROW);
            winuser::RegisterClassW(&window_class);

            Self {
                window_class_name,
                h_instance,
            }
        }
    }

    fn event_loop(&mut self) -> Self::EventLoop {
        PlatformEventLoop {}
    }

    fn set_window_position(&mut self, window_id: WindowId, x: u32, y: u32) {
        unsafe {
            let mut rect = windef::RECT {
                left: 0,
                top: 0,
                right: 0,
                bottom: 0,
            };
            winuser::GetWindowRect(window_id.raw() as windef::HWND, &mut rect);
            let width = rect.right - rect.left;
            let height = rect.bottom - rect.top;

            winuser::MoveWindow(
                window_id.raw() as windef::HWND,
                x as i32,
                y as i32,
                width,
                height,
                FALSE,
            );
        }
    }
    fn set_window_dimensions(&mut self, window_id: WindowId, width: u32, height: u32) {
        unsafe {
            let mut rect = windef::RECT {
                left: 0,
                top: 0,
                right: 0,
                bottom: 0,
            };
            winuser::GetWindowRect(window_id.raw() as windef::HWND, &mut rect);
            winuser::MoveWindow(
                window_id.raw() as windef::HWND,
                rect.left,
                rect.top,
                width as i32,
                height as i32,
                FALSE,
            );
        }
    }
    fn set_window_title(&mut self, window_id: WindowId, title: &str) {
        let title = win32_string(title);
        unsafe {
            winuser::SetWindowTextW(window_id.raw() as windef::HWND, title.as_ptr());
        }
    }
    fn minimize_window(&mut self, window_id: WindowId) {
        unsafe {
            winuser::ShowWindow(window_id.raw() as windef::HWND, winuser::SW_MINIMIZE);
        }
    }
    fn maximize_window(&mut self, window_id: WindowId) {
        unsafe {
            winuser::ShowWindow(window_id.raw() as windef::HWND, winuser::SW_MAXIMIZE);
        }
    }
    fn fullscreen_window(&mut self, window_id: WindowId) {
        unsafe {
            let hwnd = window_id.raw() as windef::HWND;
            let screen_width = winuser::GetSystemMetrics(winuser::SM_CXSCREEN);
            let screen_height = winuser::GetSystemMetrics(winuser::SM_CYSCREEN);
            winuser::SetWindowLongPtrW(
                hwnd,
                winuser::GWL_STYLE,
                (winuser::WS_VISIBLE | winuser::WS_POPUP) as isize,
            );
            let mut rect = windef::RECT {
                left: 0,
                top: 0,
                right: 0,
                bottom: 0,
            };
            winuser::GetWindowRect(window_id.raw() as windef::HWND, &mut rect);
            winuser::MoveWindow(
                window_id.raw() as windef::HWND,
                rect.left,
                rect.top,
                screen_width as i32,
                screen_height as i32,
                FALSE,
            );
        }
    }
    fn restore_window(&mut self, window_id: WindowId) {
        unsafe {
            winuser::ShowWindow(window_id.raw() as windef::HWND, winuser::SW_RESTORE);
        }
    }
    fn close_window(&mut self, window_id: WindowId) {
        unsafe {
            winuser::CloseWindow(window_id.raw() as windef::HWND);
        }
    }

    fn redraw_window(&mut self, window_id: WindowId) {
        unsafe {
            if !WINDOWS_TO_REDRAW.contains(&window_id) {
                WINDOWS_TO_REDRAW.push(window_id);
            }
        }
    }

    fn set_mouse_position(&mut self, x: u32, y: u32) {
        unsafe {
            winuser::SetCursorPos(x as i32, y as i32);
        }
    }

    fn new_window(&mut self, window_parameters: &WindowParameters) -> WindowId {
        unsafe {
            let extended_style = winuser::WS_EX_APPWINDOW;
            let window_style =
                winuser::WS_OVERLAPPEDWINDOW | winuser::WS_VISIBLE | winuser::CS_OWNDC;
            let title = win32_string(&window_parameters.title);

            let (x, y) = if let Some(position) = window_parameters.position {
                (position.0 as i32, position.1 as i32)
            } else {
                (winuser::CW_USEDEFAULT, winuser::CW_USEDEFAULT)
            };

            let (width, height) = window_parameters.dimensions.map_or(
                (winuser::CW_USEDEFAULT, winuser::CW_USEDEFAULT),
                |d| {
                    let mut rect = windef::RECT {
                        left: 0,
                        top: 0,
                        right: d.0 as i32,
                        bottom: d.1 as i32,
                    };

                    // Windows will provide a window with a smaller client area than desired (because it includes borders in the window size).
                    // This call returns an adjusted rect accounting for the borders based on the window_style.
                    winuser::AdjustWindowRectEx(
                        &mut rect,
                        window_style,
                        minwindef::FALSE,
                        extended_style,
                    );

                    (rect.right - rect.left, rect.bottom - rect.top)
                },
            );

            let window_handle = winuser::CreateWindowExW(
                extended_style,
                self.window_class_name.as_ptr(),
                title.as_ptr(),
                window_style,
                x as i32,
                y as i32,
                width,
                height,
                null_mut(),
                null_mut(),
                self.h_instance,
                null_mut(),
            );

            let device_context = winuser::GetDC(window_handle);
            let window_id = WindowId::new_with_device_context(
                window_handle as *mut std::ffi::c_void,
                device_context as *mut std::ffi::c_void,
            );
            WINDOWS_TO_REDRAW.push(window_id); // Send the window an initial Draw event.
            window_id
        }
    }

    fn quit(&self) {
        unsafe {
            winuser::PostQuitMessage(0);
        }
    }

    fn set_cursor(&mut self, cursor: Cursor) {
        unsafe {
            // List of cursors here:
            // https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-loadcursora
            let cursor = match cursor {
                Cursor::Arrow => winuser::LoadCursorW(null_mut(), winuser::IDC_ARROW),
                Cursor::IBeam => winuser::LoadCursorW(null_mut(), winuser::IDC_IBEAM),

                // There's no default for this on Windows
                Cursor::PointingHand => winuser::LoadCursorW(null_mut(), winuser::IDC_ARROW),
                Cursor::OpenHand => winuser::LoadCursorW(null_mut(), winuser::IDC_HAND),

                // There's no default for this on Windows
                Cursor::ClosedHand => winuser::LoadCursorW(null_mut(), winuser::IDC_HAND),
            };

            winuser::SetCursor(super::application_windows::CURRENT_CURSOR);

            // This is a workaround.
            // The cursor doesn't immediately update because the WM_SETCURSOR event isn't
            // sent immediately. By setting the position here, the mouse moves and WM_SETCURSOR is sent.
            let mut position = windef::POINT { x: 0, y: 0 };
            winuser::GetCursorPos(&mut position);
            winuser::SetCursorPos(position.x, position.y);

            CURRENT_CURSOR = cursor;
        }
    }
    fn hide_cursor(&mut self) {
        unsafe {
            winuser::ShowCursor(FALSE);
        }
    }
    fn show_cursor(&mut self) {
        unsafe {
            winuser::ShowCursor(TRUE);
        }
    }
}

pub struct PlatformEventLoop {}

impl PlatformEventLoopTrait for PlatformEventLoop {
    fn run(&mut self, callback: Box<dyn FnMut(crate::Event)>) {
        super::event_loop_windows::run(callback);
    }
}
