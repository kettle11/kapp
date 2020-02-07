use super::gl_context_windows::*;
use super::utils_windows::*;
use std::io::Error;
use std::ptr::null_mut;
use winapi::shared::minwindef;
use winapi::shared::windef;
use winapi::um::libloaderapi;
use winapi::um::wingdi;
use winapi::um::winuser;

pub struct Window {
    #[allow(dead_code)]
    handle: windef::HWND,
    device: windef::HDC,
}

pub struct WindowBuilder<'a> {
    class_name: Vec<u16>,
    h_instance: minwindef::HINSTANCE,
    opengl_context: OpenGLContext,
    x: Option<u32>,
    y: Option<u32>,
    width: Option<u32>,
    height: Option<u32>,
    resizable: bool,
    title: Option<&'a str>,
}

impl<'a> WindowBuilder<'a> {
    pub fn title(&mut self, title: &'a str) -> &mut Self {
        self.title = Some(title);
        self
    }

    pub fn position(&mut self, x: u32, y: u32) -> &mut Self {
        self.x = Some(x);
        self.y = Some(y);
        self
    }
    pub fn dimensions(&mut self, width: u32, height: u32) -> &mut Self {
        self.width = Some(width);
        self.height = Some(height);
        self
    }

    pub fn build(&self) -> Result<Window, Error> {
        unsafe {
            let title = win32_string(self.title.unwrap_or("Untitled"));

            let x = self.x.map(|x| x as i32).unwrap_or(winuser::CW_USEDEFAULT);
            let y = self.y.map(|y| y as i32).unwrap_or(winuser::CW_USEDEFAULT);
            let width = self
                .width
                .map(|w| w as i32)
                .unwrap_or(winuser::CW_USEDEFAULT);
            let height = self
                .height
                .map(|h| h as i32)
                .unwrap_or(winuser::CW_USEDEFAULT);

            let window_handle = winuser::CreateWindowExW(
                winuser::WS_EX_APPWINDOW,
                self.class_name.as_ptr(),
                title.as_ptr(),
                winuser::WS_OVERLAPPEDWINDOW | winuser::WS_VISIBLE,
                x,
                y,
                width,
                height,
                null_mut(),
                null_mut(),
                self.h_instance,
                null_mut(),
            );
            let window_device = winuser::GetDC(window_handle);
            error_if_null(window_device, false)?;

            // make that match the device context's current pixel format
            error_if_false(
                wingdi::SetPixelFormat(
                    window_device,
                    self.opengl_context.pixel_format_id,
                    &self.opengl_context.pixel_format_descriptor,
                ),
                false,
            )?;

            // When a window is constructed, make it current.
            error_if_false(
                wingdi::wglMakeCurrent(window_device, self.opengl_context.context_ptr),
                false,
            )?;

            Ok(Window {
                handle: window_handle,
                device: window_device,
            })
        }
    }
}

pub struct WindowManagerBuilder {
    color_bits: u8,
    alpha_bits: u8,
    depth_bits: u8,
    stencil_bits: u8,
    samples: u8,
    srgb: bool,
}

impl WindowManagerBuilder {
    pub fn bits(
        &mut self,
        color_bits: u8,
        alpha_bits: u8,
        depth_bits: u8,
        stencil_bits: u8,
    ) -> &mut Self {
        self.color_bits = color_bits;
        self.alpha_bits = alpha_bits;
        self.depth_bits = depth_bits;
        self.stencil_bits = stencil_bits;
        self
    }
    pub fn color_bits(&mut self, bits: u8) -> &mut Self {
        self.color_bits = bits;
        self
    }

    pub fn alpha_bits(&mut self, bits: u8) -> &mut Self {
        self.alpha_bits = bits;
        self
    }

    pub fn depth_bits(&mut self, bits: u8) -> &mut Self {
        self.depth_bits = bits;
        self
    }

    pub fn stencil_bits(&mut self, bits: u8) -> &mut Self {
        self.stencil_bits = bits;
        self
    }

    /// Sets the MSAA samples.
    /// Set this to a power of 2.
    /// With an Nvidia card on Windows I was unable to set this below 2.
    pub fn samples(&mut self, samples: u8) -> &mut Self {
        self.samples = samples;
        self
    }

    /// This sets if the backbuffer for the windows will be in sRGB color space... or it would if drivers respected it.
    /// Unfortunately this flag does nothing as tested on Windows with an Nvidia GPU.
    /// In that case backbuffer was set to sRGB colorspace.
    pub fn srgb(&mut self, srgb: bool) -> &mut Self {
        self.srgb = srgb;
        self
    }

    pub fn build(&self) -> Result<WindowManager, Error> {
        unsafe {
            // Register the window class.
            let class_name = win32_string("windowing_rust");
            let h_instance = libloaderapi::GetModuleHandleW(null_mut());

            let window_class = winuser::WNDCLASSW {
                style: 0,
                lpfnWndProc: Some(super::event_loop_windows::window_callback),
                cbClsExtra: 0,
                cbWndExtra: 0,
                hInstance: h_instance,
                hIcon: null_mut(),
                hCursor: null_mut(),
                hbrBackground: null_mut(),
                lpszMenuName: null_mut(),
                lpszClassName: class_name.as_ptr(),
            };
            winuser::RegisterClassW(&window_class);

            let opengl_context = new_opengl_context(
                h_instance,
                &class_name,
                self.color_bits,
                self.alpha_bits,
                self.depth_bits,
                self.stencil_bits,
                self.samples,
                self.srgb,
                false,
            )?;
            WindowManager::setup_gl()?;
            Ok(WindowManager {
                class_name,
                h_instance,
                opengl_context,
            })
        }
    }
}
pub struct WindowManager {
    class_name: Vec<u16>,
    h_instance: minwindef::HINSTANCE,
    opengl_context: OpenGLContext,
}

impl WindowManager {
    pub fn new() -> WindowManagerBuilder {
        WindowManagerBuilder {
            color_bits: 32,
            alpha_bits: 8,
            depth_bits: 16,
            stencil_bits: 0,
            samples: 1,
            srgb: true,
        }
    }

    fn setup_gl() -> Result<(), Error> {
        unsafe {
            // Load swap interval for Vsync
            let function_pointer = wingdi::wglGetProcAddress(
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

    pub fn new_window<'a>(&mut self) -> WindowBuilder<'a> {
        WindowBuilder {
            class_name: self.class_name.clone(),
            h_instance: self.h_instance,
            opengl_context: self.opengl_context.clone(),
            x: None,
            y: None,
            width: None,
            height: None,
            resizable: true,
            title: None,
        }
    }

    pub fn make_current(&self, window: &Window) -> Result<(), Error> {
        unsafe {
            error_if_false(
                wingdi::wglMakeCurrent(window.device, self.opengl_context.context_ptr),
                false,
            )
        }
    }

    pub fn swap_buffers(&self, window: &Window) {
        unsafe {
            wingdi::SwapBuffers(window.device);
        }
    }

    // This belongs to the window builder because the OpenGL context must be constructed first
    // and the window builder creates the context.
    pub fn gl_loader(&self) -> Box<dyn FnMut(&'static str) -> *const std::ffi::c_void> {
        unsafe {
            let opengl_module = libloaderapi::LoadLibraryA(
                std::ffi::CString::new("opengl32.dll").unwrap().as_ptr(),
            );
            Box::new(move |s| {
                let name = std::ffi::CString::new(s).unwrap();
                let mut result = wingdi::wglGetProcAddress(name.as_ptr() as *const i8)
                    as *const std::ffi::c_void;
                if result.is_null() {
                    // Functions were part of OpenGL1 need to be loaded differently.
                    result = libloaderapi::GetProcAddress(opengl_module, name.as_ptr() as *const i8)
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
