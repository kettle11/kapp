/// All windows share a pixel format and an OpenGlContext.
extern crate winapi;
use super::gl_context_windows::*;
use super::keys_windows::virtual_keycode_to_key;
use super::utils_windows::*;
use crate::events::*;
use crate::Key;
use std::io::Error;
use std::ptr::null_mut;

use winapi::shared::minwindef::{HINSTANCE, HIWORD, LOWORD, LPARAM, LRESULT, UINT, WPARAM};
use winapi::shared::windef::{HDC, HWND};
use winapi::shared::windowsx::{GET_X_LPARAM, GET_Y_LPARAM};
use winapi::um::libloaderapi::{GetModuleHandleW, GetProcAddress, LoadLibraryA};
use winapi::um::wingdi::{wglGetProcAddress, wglMakeCurrent, SetPixelFormat, SwapBuffers};
use winapi::um::winuser;
use winapi::um::winuser::{
    CreateWindowExW, DefWindowProcW, DispatchMessageW, GetDC, PeekMessageW, RegisterClassW,
    TranslateMessage, CW_USEDEFAULT, MSG, PM_REMOVE, WM_QUIT, WNDCLASSW, WS_EX_APPWINDOW,
    WS_OVERLAPPEDWINDOW, WS_VISIBLE,
};

type Callback = dyn 'static + FnMut(Event);
static mut PROGRAM_CALLBACK: Option<Box<Callback>> = None;

pub fn run<T>(callback: T)
where
    T: 'static + FnMut(Event),
{
    unsafe {
        PROGRAM_CALLBACK = Some(Box::new(callback));

        let mut message: MSG = std::mem::zeroed();
        while message.message != WM_QUIT {
            while PeekMessageW(&mut message, null_mut(), 0, 0, PM_REMOVE) > 0 {
                TranslateMessage(&message as *const MSG);
                DispatchMessageW(&message as *const MSG);
            }

            if let Some(program_callback) = PROGRAM_CALLBACK.as_mut() {
                program_callback(Event::Draw);
            }
        }
    }
}

unsafe extern "system" fn window_callback(
    hwnd: HWND,
    u_msg: UINT,
    w_param: WPARAM,
    l_param: LPARAM,
) -> LRESULT {
    match u_msg {
        winuser::WM_KEYDOWN => produce_event(process_key_down(w_param, l_param)),
        winuser::WM_KEYUP => produce_event(process_key_up(w_param, l_param)),
        winuser::WM_SIZE => {
            match w_param {
                winuser::SIZE_MAXIMIZED => produce_event(Event::MaximizedWindow),
                winuser::SIZE_MINIMIZED => produce_event(Event::MinimizedWindow),
                winuser::SIZE_RESTORED => {
                    /* Quote from the docs: "The window has been resized, but
                    neither the SIZE_MINIMIZED nor SIZE_MAXIMIZED value applies" */
                    produce_event(process_resize_event(l_param));
                    // While resizing the OS directly calls window_callback and does not call the typical event loop.
                    // To redraw the window smoothly Event::Draw is passed in here.
                    produce_event(Event::Draw);
                }
                _ => {}
            }
        }
        winuser::WM_LBUTTONDOWN => produce_event(Event::MouseDown {
            button: MouseButton::Left,
        }),
        winuser::WM_MBUTTONDOWN => produce_event(Event::MouseDown {
            button: MouseButton::Middle,
        }),
        winuser::WM_RBUTTONDOWN => produce_event(Event::MouseDown {
            button: MouseButton::Right,
        }),
        winuser::WM_LBUTTONUP => produce_event(Event::MouseUp {
            button: MouseButton::Left,
        }),
        winuser::WM_MBUTTONUP => produce_event(Event::MouseUp {
            button: MouseButton::Middle,
        }),
        winuser::WM_RBUTTONUP => produce_event(Event::MouseUp {
            button: MouseButton::Right,
        }),
        winuser::WM_MOUSEMOVE => produce_event(process_mouse_move_event(hwnd, l_param)),
        _ => {}
    }
    // DefWindowProcW is the default Window event handler.
    DefWindowProcW(hwnd, u_msg, w_param, l_param)
}

fn produce_event(event: Event) {
    unsafe {
        if let Some(program_callback) = PROGRAM_CALLBACK.as_mut() {
            program_callback(event)
        }
    }
}

// https://docs.microsoft.com/en-us/windows/win32/inputdev/wm-mousemove
fn process_mouse_move_event(_hwnd: HWND, l_param: LPARAM) -> Event {
    let x = GET_X_LPARAM(l_param);
    let y = GET_Y_LPARAM(l_param);

    Event::MouseMoved {
        x: x as f32,
        y: y as f32,
    }
}
// https://docs.microsoft.com/en-us/windows/win32/winmsg/wm-size
fn process_resize_event(l_param: LPARAM) -> Event {
    let width = LOWORD(l_param as u32) as u32;
    let height = HIWORD(l_param as u32) as u32;
    Event::ResizedWindow { width, height }
}

fn process_key_down(w_param: WPARAM, l_param: LPARAM) -> Event {
    let (scancode, key) = process_key_event(w_param, l_param);
    Event::KeyDown { key, scancode }
}

fn process_key_up(w_param: WPARAM, l_param: LPARAM) -> Event {
    let (scancode, key) = process_key_event(w_param, l_param);
    Event::KeyUp { key, scancode }
}

fn process_key_event(w_param: WPARAM, l_param: LPARAM) -> (UINT, Key) {
    let scancode = ((l_param >> 16) & 16) as UINT; // bits 16-23 represent the scancode
    let _extended = (l_param & (1 << 24)) != 0; // bit 24 represents if its an extended key
    let key = virtual_keycode_to_key(w_param as _);
    (scancode, key)
}

pub struct WindowManager {
    class_name: Vec<u16>,
    h_instance: HINSTANCE,
    opengl_context: OpenGLContext,
}

pub struct Window {
    #[allow(dead_code)]
    handle: HWND,
    device: HDC,
}

impl WindowManager {
    pub fn new() -> Result<Self, Error> {
        unsafe {
            // Register the window class.
            let class_name = win32_string("windowing_rust");
            let h_instance = GetModuleHandleW(null_mut());

            let window_class = WNDCLASSW {
                style: 0,
                lpfnWndProc: Some(window_callback),
                cbClsExtra: 0,
                cbWndExtra: 0,
                hInstance: h_instance,
                hIcon: null_mut(),
                hCursor: null_mut(),
                hbrBackground: null_mut(),
                lpszMenuName: null_mut(),
                lpszClassName: class_name.as_ptr(),
            };
            RegisterClassW(&window_class);

            let opengl_context =
                new_opengl_context(h_instance, &class_name, 32, 8, 16, 0, 2, false)?;
            Self::setup_gl()?;
            Ok(Self {
                class_name,
                h_instance,
                opengl_context,
            })
        }
    }

    fn setup_gl() -> Result<(), Error> {
        unsafe {
            // Load swap interval for Vsync
            let function_pointer = wglGetProcAddress(
                std::ffi::CString::new("wglSwapIntervalEXT")
                    .unwrap()
                    .as_ptr() as *const i8,
            );

            if function_pointer.is_null() {
                println!("Could not find wglSwapIntervalEXT");
                return Err(Error::last_os_error());
            } else {
                wglSwapIntervalEXT_ptr = function_pointer as *const std::ffi::c_void;
            }

            // Default to Vsync enabled
            if !wglSwapIntervalEXT(1) {
                return Err(Error::last_os_error());
            }
        }
        Ok(())
    }

    pub fn new_window(
        &mut self,
        title: &str,
        width: Option<u32>,
        height: Option<u32>,
    ) -> Result<Window, Error> {
        unsafe {
            let title = win32_string(title);

            let width = width.map(|w| w as i32).unwrap_or(CW_USEDEFAULT);
            let height = height.map(|h| h as i32).unwrap_or(CW_USEDEFAULT);

            let window_handle = CreateWindowExW(
                WS_EX_APPWINDOW,
                self.class_name.as_ptr(),
                title.as_ptr(),
                WS_OVERLAPPEDWINDOW | WS_VISIBLE,
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                width,
                height,
                null_mut(),
                null_mut(),
                self.h_instance,
                null_mut(),
            );
            let window_device = GetDC(window_handle);
            error_if_null(window_device, false)?;

            // make that match the device context's current pixel format
            error_if_false(
                SetPixelFormat(
                    window_device,
                    self.opengl_context.pixel_format_id,
                    &self.opengl_context.pixel_format_descriptor,
                ),
                false,
            )?;

            // When a window is constructed, make it current.
            wglMakeCurrent(window_device, self.opengl_context.context_ptr);

            Ok(Window {
                handle: window_handle,
                device: window_device,
            })
        }
    }

    pub fn make_current(&self, window: &Window) -> Result<(), Error> {
        unsafe {
            error_if_false(
                wglMakeCurrent(window.device, self.opengl_context.context_ptr),
                false,
            )
        }
    }

    pub fn swap_buffers(&self, window: &Window) {
        unsafe {
            SwapBuffers(window.device);
        }
    }

    // This belongs to the window builder because the OpenGL context must be constructed first
    // and the window builder creates the context.
    pub fn gl_loader(&self) -> Box<dyn FnMut(&'static str) -> *const std::ffi::c_void> {
        unsafe {
            let opengl_module =
                LoadLibraryA(std::ffi::CString::new("opengl32.dll").unwrap().as_ptr());
            Box::new(move |s| {
                let name = std::ffi::CString::new(s).unwrap();
                let mut result =
                    wglGetProcAddress(name.as_ptr() as *const i8) as *const std::ffi::c_void;
                if result.is_null() {
                    // Functions were part of OpenGL1 need to be loaded differently.
                    result = GetProcAddress(opengl_module, name.as_ptr() as *const i8)
                        as *const std::ffi::c_void;
                }
                /*
                if result.is_null() {
                    println!("FAILED TO LOAD: {}", s);
                } else {
                    println!("Loaded: {}", s);
                }
                */
                result
            })
        }
    }
}

// This is a C extension function requested on load.
#[allow(non_upper_case_globals)]
static mut wglSwapIntervalEXT_ptr: *const std::ffi::c_void = std::ptr::null();
#[allow(non_upper_case_globals)]
#[allow(non_snake_case)]
fn wglSwapIntervalEXT(i: std::os::raw::c_int) -> bool {
    unsafe {
        std::mem::transmute::<_, extern "system" fn(std::os::raw::c_int) -> bool>(
            wglSwapIntervalEXT_ptr,
        )(i)
    }
}
