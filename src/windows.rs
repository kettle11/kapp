/// All windows share a pixel format and an OpenGlContext.
extern crate gl;
extern crate winapi;

use crate::keys_windows::virtual_keycode_to_key;
use crate::Key;

use std::ffi::OsStr;
use std::io::Error;
use std::iter::once;
use std::os::windows::prelude::*;
use std::ptr::null_mut;

use winapi::shared::minwindef::{HINSTANCE, HIWORD, LOWORD, LPARAM, LRESULT, UINT, WPARAM};
use winapi::shared::windef::{HDC, HGLRC, HWND};
use winapi::shared::windowsx::{GET_X_LPARAM, GET_Y_LPARAM};
use winapi::um::libloaderapi::{GetModuleHandleW, GetProcAddress, LoadLibraryA};
use winapi::um::wingdi::{
    wglCreateContext, wglGetProcAddress, wglMakeCurrent, ChoosePixelFormat, SetPixelFormat,
    SwapBuffers, PFD_DOUBLEBUFFER, PFD_DRAW_TO_WINDOW, PFD_MAIN_PLANE, PFD_SUPPORT_OPENGL,
    PFD_TYPE_RGBA, PIXELFORMATDESCRIPTOR,
};
use winapi::um::winuser;
use winapi::um::winuser::{
    CreateWindowExW, DefWindowProcW, DispatchMessageW, GetDC, PeekMessageW, RegisterClassW,
    TranslateMessage, CW_USEDEFAULT, MSG, PM_REMOVE, WM_QUIT, WNDCLASSW, WS_EX_APPWINDOW,
    WS_OVERLAPPEDWINDOW, WS_VISIBLE,
};

pub enum MouseButton {
    Left,
    Right,
    Middle,
}

pub enum Event {
    Draw,
    KeyDown {
        key: Key,
        scancode: u32,
    },
    MinimizedWindow,
    MaximizedWindow,
    ResizedWindow {
        width: u32,
        height: u32,
    },
    MouseMoved {
        x: f32,
        y: f32,
    },
    MouseDown {
        button: MouseButton,
    },
    MouseUp {
        button: MouseButton,
    },
    #[doc(hidden)]
    __Nonexhaustive, // More events will be added with time.
}

type Callback = dyn 'static + FnMut(Event);
static mut PROGRAM_CALLBACK: Option<Box<Callback>> = None;

unsafe extern "system" fn window_callback(
    hwnd: HWND,
    u_msg: UINT,
    w_param: WPARAM,
    l_param: LPARAM,
) -> LRESULT {
    match u_msg {
        winuser::WM_KEYDOWN => produce_event(process_key_event(w_param, l_param)),
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
fn process_mouse_move_event(hwnd: HWND, l_param: LPARAM) -> Event {
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

fn process_key_event(w_param: WPARAM, l_param: LPARAM) -> Event {
    let scancode = ((l_param >> 16) & 16) as UINT; // bits 16-23 represent the scancode
    let _extended = (l_param & (1 << 24)) != 0; // bit 24 represents if its an extended key
    let key = virtual_keycode_to_key(w_param as _);
    Event::KeyDown { key, scancode }
}

fn win32_string(value: &str) -> Vec<u16> {
    OsStr::new(value).encode_wide().chain(once(0)).collect()
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

pub struct WindowManager {
    device: Option<HDC>,
    gl_device: Option<HGLRC>,
    class_name: Vec<u16>,
    h_instance: HINSTANCE,
    pixel_format: Option<i32>,
    pixel_format_desciptor: Option<PIXELFORMATDESCRIPTOR>,
}

pub struct Window {
    handle: HWND,
    device: HDC,
}

impl WindowManager {
    pub fn new() -> Self {
        unsafe {
            // Register the window class.
            let class_name = win32_string("windowing_rust");
            let h_instance = GetModuleHandleW(std::ptr::null_mut());

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

            Self {
                device: None,
                gl_device: None,
                pixel_format: None,
                class_name,
                h_instance,
                pixel_format_desciptor: None,
            }
        }
    }

    fn setup_pixel_format(&mut self) -> Result<i32, Error> {
        let pfd = PIXELFORMATDESCRIPTOR {
            nSize: std::mem::size_of::<PIXELFORMATDESCRIPTOR>() as u16, // size of this pfd
            nVersion: 1,                                                // version number
            dwFlags: PFD_DRAW_TO_WINDOW | PFD_SUPPORT_OPENGL | PFD_DOUBLEBUFFER,
            iPixelType: PFD_TYPE_RGBA, // RGBA type
            cColorBits: 24,            // 24-bit color depth
            cRedBits: 24,              // 24-bit color depth
            cRedShift: 0,              // color bits ignored
            cGreenBits: 0,
            cGreenShift: 0,
            cBlueBits: 0,
            cBlueShift: 0,
            cAlphaBits: 0,
            cAlphaShift: 0,
            cAccumBits: 0,
            cAccumRedBits: 0,
            cAccumGreenBits: 0,
            cAccumBlueBits: 0,
            cAccumAlphaBits: 0,
            cDepthBits: 32,
            cStencilBits: 0,
            cAuxBuffers: 0,
            iLayerType: PFD_MAIN_PLANE,
            bReserved: 0,
            dwLayerMask: 0,
            dwVisibleMask: 0,
            dwDamageMask: 0,
        };
        let device = self.device.unwrap();
        // This will find the closest pixel format match.
        let i_pixel_format = unsafe { ChoosePixelFormat(device, &pfd) };
        if i_pixel_format == 0 {
            return Err(Error::last_os_error());
        } else {
            self.pixel_format_desciptor = Some(pfd);
            Ok(i_pixel_format)
        }
    }

    fn setup_gl(&mut self) -> Result<(), Error> {
        unsafe {
            let device = self.device.unwrap();

            let gl_device = wglCreateContext(device);
            if gl_device.is_null() {
                Err(Error::last_os_error())
            } else {
                wglMakeCurrent(device, gl_device);

                let opengl_module =
                    LoadLibraryA(std::ffi::CString::new("opengl32.dll").unwrap().as_ptr());
                gl::load_with(|s| {
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
                });

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
                self.gl_device = Some(gl_device);
                Ok(())
            }
        }
    }

    fn new_device(&mut self, h_wnd: HWND) -> Result<HDC, Error> {
        unsafe {
            let device = GetDC(h_wnd);
            if device.is_null() {
                Err(Error::last_os_error())
            } else {
                Ok(device)
            }
        }
    }

    pub fn new_window(&mut self, title: &str) -> Result<Window, Error> {
        unsafe {
            let title = win32_string(title);

            let handle = CreateWindowExW(
                WS_EX_APPWINDOW,
                self.class_name.as_ptr(),
                title.as_ptr(),
                WS_OVERLAPPEDWINDOW | WS_VISIBLE,
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                null_mut(),
                null_mut(),
                self.h_instance,
                null_mut(),
            );
            let device = self.new_device(handle)?;
            if self.device.is_none() {
                self.device = Some(device);
            }

            if self.pixel_format.is_none() {
                self.pixel_format = Some(self.setup_pixel_format()?);
            }
            // make that match the device context's current pixel format
            if SetPixelFormat(
                device,
                self.pixel_format.unwrap(),
                self.pixel_format_desciptor.as_ref().unwrap(),
            ) == 0
            {
                return Err(Error::last_os_error());
            }

            if self.gl_device.is_none() {
                self.setup_gl()?;
            }
            if handle.is_null() {
                Err(Error::last_os_error())
            } else {
                Ok(Window { handle, device })
            }
        }
    }

    pub fn make_current(&self, window: &Window) -> Result<(), Error> {
        unsafe {
            if wglMakeCurrent(window.device, self.gl_device.unwrap()) == 0 {
                Err(Error::last_os_error())
            } else {
                Ok(())
            }
        }
    }

    pub fn swap_buffers(&self, window: &Window) {
        unsafe {
            SwapBuffers(window.device);
        }
    }
}

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
