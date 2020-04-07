use std::io::Error;
use std::mem::size_of;
use std::os::raw::{c_float, c_int, c_uint, c_void};
use std::ptr::null_mut;

use winapi::shared::minwindef;
use winapi::shared::minwindef::{FALSE, HINSTANCE, HMODULE, TRUE};
use winapi::shared::windef;
use winapi::um::libloaderapi;
use winapi::um::wingdi;
use winapi::um::winuser;
mod utils_windows;
use utils_windows::*;

pub struct GLContextBuilder {
    samples: u8,
    color_bits: u8,
    alpha_bits: u8,
    depth_bits: u8,
    stencil_bits: u8,
    srgb: bool,
}

pub struct GLContext {
    context_ptr: windef::HGLRC,
    pixel_format_id: i32,
    _pixel_format_descriptor: wingdi::PIXELFORMATDESCRIPTOR,
    opengl_module: HMODULE,
    current_window: Option<windef::HWND>,
    window_device_context: Option<windef::HDC>,
}

// This isn't really true because make_current must be called after GLContext is passed to another thread.
// A solution would be to wrap this is an object to send to another thread, and the unwrap
// calls make_current.
unsafe impl Send for GLContext {}

impl GLContextBuilder {
    pub fn samples(&mut self, samples: u8) -> &mut Self {
        self.samples = samples;
        self
    }

    pub fn build(&self) -> Result<GLContext, ()> {
        Ok(new_opengl_context(
            self.color_bits,
            self.alpha_bits,
            self.depth_bits,
            self.stencil_bits,
            self.samples,
            self.srgb,
            true,
        )
        .unwrap())
    }
}

impl GLContext {
    pub fn new() -> GLContextBuilder {
        GLContextBuilder {
            samples: 1,
            color_bits: 24,
            alpha_bits: 8,
            depth_bits: 24,
            stencil_bits: 8,
            srgb: false,
        }
    }

    pub fn set_window(
        &mut self,
        window: Option<&kettlewin_platform_common::WindowId>,
    ) -> Result<(), Error> {
        unsafe {
            self.set_window_raw(window.map(|w| w.raw() as *mut std::ffi::c_void))?;
        }
        Ok(())
    }

    pub fn set_window_raw(&mut self, window: Option<*mut std::ffi::c_void>) -> Result<(), Error> {
        let window_handle = window.unwrap_or(null_mut()) as windef::HWND;

        if let Some(device_context) = self.window_device_context {
            unsafe {
                winuser::ReleaseDC(self.current_window.unwrap(), device_context);
            }
        }

        unsafe {
            let window_device_context = winuser::GetDC(window_handle);
            let pixel_format_descriptor: wingdi::PIXELFORMATDESCRIPTOR = std::mem::zeroed();

            // This will error if the window was previously set with an incompatible
            // pixel format.
            error_if_false(
                wingdi::SetPixelFormat(
                    window_device_context,
                    self.pixel_format_id,
                    &pixel_format_descriptor,
                ),
                false,
            )?;

            error_if_false(
                wingdi::wglMakeCurrent(window_device_context, self.context_ptr),
                false,
            )?;

            wglSwapIntervalEXT(1); // Everytime a device context is requested, vsync must be updated.
            self.window_device_context = Some(window_device_context);
        }

        self.current_window = Some(window_handle);
        Ok(())
    }

    // Updates the backbuffer of the target when it resizes
    pub fn update_target(&self) {
        //unimplemented!()
    }

    // Is this behavior correct? Does it really work if called from another thread?
    pub fn make_current(&self) {
        if let Some(window_device_context) = self.window_device_context {
            unsafe {
                // This check may only be masking an issue that occurs if wglMakeCurrent is called
                // too frequently.
                // Is there some sort of race condition with wglMakeCurrent?
                if wingdi::wglGetCurrentContext() != self.context_ptr {
                    error_if_false(
                        wingdi::wglMakeCurrent(window_device_context, self.context_ptr),
                        false,
                    )
                    .unwrap();
                }
            }
        }
    }

    pub fn gl_loader_c_string(&self) -> Box<dyn FnMut(*const i8) -> *const std::ffi::c_void> {
        let opengl_module = self.opengl_module;
        Box::new(move |s| unsafe {
            let name = std::ffi::CStr::from_ptr(s);
            Self::get_proc_address_inner(opengl_module, (&name).to_str().unwrap())
        })
    }

    pub fn swap_buffers(&self) {
        unsafe {
            if let Some(window_device_context) = self.window_device_context {
                wingdi::SwapBuffers(window_device_context);
            }
        }
    }

    pub fn get_proc_address(&self, address: &str) -> *const core::ffi::c_void {
        Self::get_proc_address_inner(self.opengl_module, address)
    }

    fn get_proc_address_inner(opengl_module: HMODULE, address: &str) -> *const core::ffi::c_void {
        unsafe {
            let name = std::ffi::CString::new(address).unwrap();
            let mut result =
                wingdi::wglGetProcAddress(name.as_ptr() as *const i8) as *const std::ffi::c_void;
            if result.is_null() {
                // Functions that were part of OpenGL1 need to be loaded differently.
                result = libloaderapi::GetProcAddress(opengl_module, name.as_ptr() as *const i8)
                    as *const std::ffi::c_void;
            }

            /*
            if result.is_null() {
                println!("FAILED TO LOAD: {}", address);
            } else {
                println!("Loaded: {} {:?}", address, result);
            }
            */
            result
        }
    }

    pub fn get_swap_interval(&self) -> i32 {
        wglGetSwapIntervalEXT()
    }
}

impl Drop for GLContext {
    fn drop(&mut self) {
        unimplemented!()
    }
}

/// Creates an OpenGL context.
/// h_instance is the parent module's h_instance
/// class_name is the parent class's name
/// panic_if_fail will crash the program with a useful callstack if something goes wrong
/// color bits and alpha bits should add up to 32
pub fn new_opengl_context(
    color_bits: u8,
    alpha_bits: u8,
    depth_bits: u8,
    stencil_bits: u8,
    msaa_samples: u8,
    srgb: bool,
    panic_if_fail: bool,
) -> Result<GLContext, Error> {
    // This function performs the following steps:
    // * First register the window class.
    // * Then create a dummy_window with that class ...
    // * Which is used to setup a dummy OpenGL context ...
    // * Which is used to load OpenGL extensions ...
    // * Which are used to set more specific pixel formats and specify an OpenGL version ...
    // * Which is used to create another dummy window ...
    // * Which is used to create the final OpenGL context!
    unsafe {
        // Register the window class.
        let window_class_name = win32_string("kettlewin_gl_window");
        let h_instance = libloaderapi::GetModuleHandleW(null_mut());

        let window_class = winuser::WNDCLASSW {
            style: 0,
            lpfnWndProc: Some(kettlewin_gl_window_callback),
            cbClsExtra: 0,
            cbWndExtra: 0,
            hInstance: h_instance,
            hIcon: null_mut(),
            hCursor: null_mut(), // This may not be what is desired. Potentially this makes it annoying to change the cursor later.
            hbrBackground: null_mut(),
            lpszMenuName: null_mut(),
            lpszClassName: window_class_name.as_ptr(),
        };
        winuser::RegisterClassW(&window_class);

        // Then create a dummy window
        let h_instance = libloaderapi::GetModuleHandleA(null_mut());
        let dummy_window = create_dummy_window(h_instance, &window_class_name);
        error_if_null(dummy_window, panic_if_fail)?;

        // DC stands for 'device context'
        // Definition of a device context:
        // https://docs.microsoft.com/en-us/windows/win32/gdi/device-contexts
        let dummy_window_dc = winuser::GetDC(dummy_window);
        error_if_null(dummy_window_dc, panic_if_fail)?;
        // Create a dummy PIXELFORMATDESCRIPTOR (PFD).
        // This PFD is based on the recommendations from here:
        // https://www.khronos.org/opengl/wiki/Creating_an_OpenGL_Context_(WGL)#Create_a_False_Context
        let mut dummy_pfd: wingdi::PIXELFORMATDESCRIPTOR = std::mem::zeroed();
        dummy_pfd.nSize = size_of::<wingdi::PIXELFORMATDESCRIPTOR>() as u16;
        dummy_pfd.nVersion = 1;
        dummy_pfd.dwFlags =
            wingdi::PFD_DRAW_TO_WINDOW | wingdi::PFD_SUPPORT_OPENGL | wingdi::PFD_DOUBLEBUFFER;
        dummy_pfd.iPixelType = wingdi::PFD_TYPE_RGBA;
        dummy_pfd.cColorBits = 32;
        dummy_pfd.cAlphaBits = 8;
        dummy_pfd.cDepthBits = 24;

        let dummy_pixel_format_id = wingdi::ChoosePixelFormat(dummy_window_dc, &dummy_pfd);

        error_if_false(dummy_pixel_format_id, panic_if_fail)?;

        error_if_false(
            wingdi::SetPixelFormat(dummy_window_dc, dummy_pixel_format_id, &dummy_pfd),
            panic_if_fail,
        )?;

        // Create the dummy OpenGL context.
        let dummy_opengl_context = wingdi::wglCreateContext(dummy_window_dc);
        error_if_null(dummy_opengl_context, panic_if_fail)?;
        error_if_false(
            wingdi::wglMakeCurrent(dummy_window_dc, dummy_opengl_context),
            panic_if_fail,
        )?;

        // Load the function to choose a pixel format.
        wglChoosePixelFormatARB_ptr =
            wgl_get_proc_address("wglChoosePixelFormatARB", panic_if_fail)?;
        // Load the function to create an OpenGL context with extra attributes.
        wglCreateContextAttribsARB_ptr =
            wgl_get_proc_address("wglCreateContextAttribsARB", panic_if_fail)?;

        // Create the second dummy window.
        let dummy_window2 = create_dummy_window(h_instance, &window_class_name);
        error_if_null(dummy_window2, panic_if_fail)?;

        // DC is 'device context'
        let dummy_window_dc2 = winuser::GetDC(dummy_window2);
        error_if_null(dummy_window_dc2, panic_if_fail)?;

        // Setup the actual pixel format we'll use.
        // Later this is where we'll specify pixel format parameters.
        // Documentation about these flags here:
        // https://www.khronos.org/registry/OpenGL/extensions/ARB/WGL_ARB_pixel_format.txt
        let pixel_attributes = vec![
            WGL_DRAW_TO_WINDOW_ARB,
            TRUE,
            WGL_SUPPORT_OPENGL_ARB,
            TRUE,
            WGL_DOUBLE_BUFFER_ARB,
            TRUE,
            WGL_PIXEL_TYPE_ARB,
            WGL_TYPE_RGBA_ARB,
            WGL_ACCELERATION_ARB,
            WGL_FULL_ACCELERATION_ARB,
            WGL_COLOR_BITS_ARB,
            color_bits as i32,
            WGL_ALPHA_BITS_ARB,
            alpha_bits as i32,
            WGL_DEPTH_BITS_ARB,
            depth_bits as i32,
            WGL_STENCIL_BITS_ARB,
            stencil_bits as i32,
            WGL_SAMPLE_BUFFERS_ARB,
            1,
            WGL_SAMPLES_ARB,
            msaa_samples as i32,
            WGL_FRAMEBUFFER_SRGB_CAPABLE_ARB,
            if srgb { TRUE } else { FALSE },
            0,
        ];

        let mut pixel_format_id = 0;
        let mut number_of_formats = 0;
        error_if_false(
            wglChoosePixelFormatARB(
                dummy_window_dc2,
                pixel_attributes.as_ptr(),
                null_mut(),
                1,
                &mut pixel_format_id,
                &mut number_of_formats,
            ),
            panic_if_fail,
        )?;
        error_if_false(number_of_formats as i32, panic_if_fail)?; // error_if_false just errors if the argument is 0, which is what we need here

        // PFD stands for 'pixel format descriptor'
        // It's unclear why this call to DescribePixelFormat is needed?
        // DescribePixelFormat fills the pfd with a description of the pixel format.
        // But why does this window need the same pixel format as the previous one?
        // Just it just need a valid pixel format?
        let mut pfd: wingdi::PIXELFORMATDESCRIPTOR = std::mem::zeroed();
        wingdi::DescribePixelFormat(
            dummy_window_dc2,
            pixel_format_id,
            size_of::<wingdi::PIXELFORMATDESCRIPTOR>() as u32,
            &mut pfd,
        );
        wingdi::SetPixelFormat(dummy_window_dc2, pixel_format_id, &pfd);

        // Finally we can create the OpenGL context!
        // Need to allow for choosing major and minor version.
        let major_version_minimum = 4;
        let minor_version_minimum = 5;
        let context_attributes = [
            WGL_CONTEXT_MAJOR_VERSION_ARB,
            major_version_minimum,
            WGL_CONTEXT_MINOR_VERSION_ARB,
            minor_version_minimum,
            WGL_CONTEXT_PROFILE_MASK_ARB,
            WGL_CONTEXT_CORE_PROFILE_BIT_ARB,
            0,
        ];

        let opengl_context = wglCreateContextAttribsARB(
            dummy_window_dc2,
            0 as windef::HGLRC, // An existing OpenGL context to share resources with. 0 means none.
            context_attributes.as_ptr(),
        );

        error_if_null(opengl_context, panic_if_fail)?;

        // Clean up all of our resources
        // It's bad that these calls only occur if all the prior steps were succesful.
        // If a program were to recover from a failure to setup an OpenGL context these resources would be leaked.
        wingdi::wglMakeCurrent(dummy_window_dc, null_mut());
        wingdi::wglDeleteContext(dummy_opengl_context);
        winuser::ReleaseDC(dummy_window, dummy_window_dc);
        winuser::DestroyWindow(dummy_window);

        error_if_false(
            wingdi::wglMakeCurrent(dummy_window_dc2, opengl_context),
            true,
        )?;

        let opengl_module =
            libloaderapi::LoadLibraryA(std::ffi::CString::new("opengl32.dll").unwrap().as_ptr());

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

        // Will the dummy window be rendererd to if no other window is made current?
        winuser::ReleaseDC(dummy_window2, dummy_window_dc2);
        winuser::DestroyWindow(dummy_window2);

        // Disconnects from current window
        // Uncommenting this line can cause intermittment crashes
        // It's unclear why, as this should just disconnect the dummy window context
        // However leaving this commented should be harmless.
        // Actually, it just improves the situation, but doesn't prevent it.
        //wingdi::wglMakeCurrent(dummy_window_dc2, null_mut());

        Ok(GLContext {
            context_ptr: opengl_context,
            pixel_format_id,
            _pixel_format_descriptor: pfd,
            opengl_module,
            current_window: None,
            window_device_context: None,
        })
    }
}

fn create_dummy_window(h_instance: HINSTANCE, class_name: &Vec<u16>) -> windef::HWND {
    let title = win32_string("Kettlewin Placeholder");

    unsafe {
        // https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw
        winuser::CreateWindowExW(
            0,                                                   // extended style Is this ok?
            class_name.as_ptr(),                                 // A class created by RegisterClass
            title.as_ptr(),                                      // window title
            winuser::WS_CLIPSIBLINGS | winuser::WS_CLIPCHILDREN, // style
            0,                                                   // x position
            0,                                                   // y position
            1,                                                   // width
            1,                                                   // height
            null_mut(),                                          // parent window
            null_mut(),                                          // menu
            h_instance,                                          // Module handle
            null_mut(),                                          // Data sent to window
        )
    }
}

pub unsafe extern "system" fn kettlewin_gl_window_callback(
    hwnd: windef::HWND,
    u_msg: minwindef::UINT,
    w_param: minwindef::WPARAM,
    l_param: minwindef::LPARAM,
) -> minwindef::LRESULT {
    // DefWindowProcW is the default Window event handler.
    winuser::DefWindowProcW(hwnd, u_msg, w_param, l_param)
}

fn wgl_get_proc_address(name: &str, panic_if_fail: bool) -> Result<*const c_void, Error> {
    let name = std::ffi::CString::new(name).unwrap();
    let result = unsafe { wingdi::wglGetProcAddress(name.as_ptr() as *const i8) as *const c_void };
    error_if_null(result, panic_if_fail)?;
    Ok(result)
}

// These definitions are based on the wglext.h header available here:
// https://www.khronos.org/registry/OpenGL/api/GL/wglext.h
#[allow(non_snake_case, non_upper_case_globals)]
static mut wglChoosePixelFormatARB_ptr: *const c_void = std::ptr::null();
#[allow(non_snake_case, non_upper_case_globals)]
fn wglChoosePixelFormatARB(
    hdc: windef::HDC,
    piAttribIList: *const c_int,
    pfAttribFList: *const c_float,
    nMaxFormats: c_uint,
    piFormats: *mut c_int,
    nNumFormats: *mut c_uint,
) -> c_int {
    unsafe {
        std::mem::transmute::<
            _,
            extern "system" fn(
                windef::HDC,
                *const c_int,
                *const c_float,
                c_uint,
                *mut c_int,
                *mut c_uint,
            ) -> c_int,
        >(wglChoosePixelFormatARB_ptr)(
            hdc,
            piAttribIList,
            pfAttribFList,
            nMaxFormats,
            piFormats,
            nNumFormats,
        )
    }
}

#[allow(non_snake_case, non_upper_case_globals)]
static mut wglCreateContextAttribsARB_ptr: *const c_void = std::ptr::null();
#[allow(non_snake_case, non_upper_case_globals)]
fn wglCreateContextAttribsARB(
    hdc: windef::HDC,
    hShareContext: windef::HGLRC,
    attribList: *const c_int,
) -> windef::HGLRC {
    unsafe {
        std::mem::transmute::<
            _,
            extern "system" fn(windef::HDC, windef::HGLRC, *const c_int) -> windef::HGLRC,
        >(wglCreateContextAttribsARB_ptr)(hdc, hShareContext, attribList)
    }
}

// Once again these are all from here:
// https://www.khronos.org/registry/OpenGL/api/GL/wglext.h
// A few are commented out that may be useful later.
static WGL_DRAW_TO_WINDOW_ARB: c_int = 0x2001;
// static WGL_DRAW_TO_BITMAP_ARB: c_int = 0x2002;
static WGL_ACCELERATION_ARB: c_int = 0x2003;
static WGL_SUPPORT_OPENGL_ARB: c_int = 0x2010;
static WGL_DOUBLE_BUFFER_ARB: c_int = 0x2011;
static WGL_PIXEL_TYPE_ARB: c_int = 0x2013;
static WGL_COLOR_BITS_ARB: c_int = 0x2014;
// static WGL_RED_BITS_ARB: c_int = 0x2015;
// static WGL_GREEN_BITS_ARB: c_int = 0x2017;
// static WGL_BLUE_BITS_ARB: c_int = 0x2019;
static WGL_ALPHA_BITS_ARB: c_int = 0x201B;
static WGL_DEPTH_BITS_ARB: c_int = 0x2022;
static WGL_STENCIL_BITS_ARB: c_int = 0x2023;
static WGL_FULL_ACCELERATION_ARB: c_int = 0x2027;
static WGL_TYPE_RGBA_ARB: c_int = 0x202B;
static WGL_SAMPLE_BUFFERS_ARB: c_int = 0x2041;
static WGL_SAMPLES_ARB: c_int = 0x2042;
static WGL_CONTEXT_MAJOR_VERSION_ARB: c_int = 0x2091;
static WGL_CONTEXT_MINOR_VERSION_ARB: c_int = 0x2092;
static WGL_CONTEXT_PROFILE_MASK_ARB: c_int = 0x9126;
static WGL_CONTEXT_CORE_PROFILE_BIT_ARB: c_int = 0x00000001;
// static WGL_CONTEXT_COMPATIBILITY_PROFILE_BIT_ARB: c_int = 0x00000002;
static WGL_FRAMEBUFFER_SRGB_CAPABLE_ARB: c_int = 0x20A9;

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

// This is a C extension function requested on load.
#[allow(non_upper_case_globals)]
static mut wglGetSwapIntervalEXT_ptr: *const std::ffi::c_void = std::ptr::null();
#[allow(non_upper_case_globals)]
#[allow(non_snake_case)]
fn wglGetSwapIntervalEXT() -> std::os::raw::c_int {
    unsafe {
        std::mem::transmute::<_, extern "system" fn() -> std::os::raw::c_int>(
            wglGetSwapIntervalEXT_ptr,
        )()
    }
}
