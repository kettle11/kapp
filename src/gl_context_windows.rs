use crate::utils_windows::*;
use std::io::Error;
use std::mem::size_of;
use std::os::raw::{c_float, c_int, c_uint, c_void};
use std::ptr::null_mut;
use winapi::shared::minwindef::{HINSTANCE, TRUE};
use winapi::shared::windef;
use winapi::um::wingdi;
use winapi::um::winuser;

pub struct OpenGLContext {
    pub context_ptr: windef::HGLRC,
    pub pixel_format_id: i32,
    pub pixel_format_descriptor: wingdi::PIXELFORMATDESCRIPTOR,
}

/// Creates an OpenGL context.
/// h_instance is the parent module's h_instance
/// class_name is the parent class's name
/// panic_if_fail will crash the program with a useful callstack if something goes wrong
/// color bits and alpha bits should add up to 32
pub fn new_opengl_context(
    h_instance: HINSTANCE,
    class_name: &Vec<u16>,
    color_bits: u8,
    alpha_bits: u8,
    depth_bits: u8,
    stencil_bits: u8,
    msaa_samples: u8,
    panic_if_fail: bool,
) -> Result<OpenGLContext, Error> {
    // This function performs the following steps:
    // * First creates a dummy_window ...
    // * Which is used to setup a dummy OpenGL context ...
    // * Which is used to load OpenGL extensions ...
    // * Which are used to set more specific pixel formats and specify an OpenGL version ...
    // * Which is used to create another dummy window ...
    // * Which is used to create the final OpenGL context
    // * Then all of the intermediate resources are discarded and the OpenGL context is returned.
    unsafe {
        let dummy_window = create_dummy_window(h_instance, class_name);
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

        let dummy_opengl_context = wingdi::wglCreateContext(dummy_window_dc);
        error_if_null(dummy_opengl_context, panic_if_fail)?;
        error_if_false(
            wingdi::wglMakeCurrent(dummy_window_dc, dummy_opengl_context),
            panic_if_fail,
        )?;

        // Load the function to choose a pixel format.
        wglChoosePixelFormatARB_ptr =
            wgl_get_proc_address("wglChoosePixelFormatARB", panic_if_fail)?;
        wglCreateContextAttribsARB_ptr =
            wgl_get_proc_address("wglCreateContextAttribsARB", panic_if_fail)?;

        let dummy_window2 = create_dummy_window(h_instance, class_name);
        error_if_null(dummy_window2, panic_if_fail)?;

        // DC is 'device context'
        let dummy_window_dc2 = winuser::GetDC(dummy_window2);
        error_if_null(dummy_window_dc2, panic_if_fail)?;

        // Setup the actual pixel format we'll use.
        // Later this is where we'll specify pixel format parameters.
        // Documentation about these flags here:
        // https://www.khronos.org/registry/OpenGL/extensions/ARB/WGL_ARB_pixel_format.txt
        let mut pixel_attributes = vec![
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
            0,
        ];

        pixel_attributes.push(WGL_FRAMEBUFFER_SRGB_CAPABLE_ARB);

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
        wingdi::wglMakeCurrent(null_mut(), null_mut());
        wingdi::wglDeleteContext(dummy_opengl_context);
        winuser::ReleaseDC(dummy_window, dummy_window_dc);
        winuser::DestroyWindow(dummy_window);

        // These resources are leaked because a window is needed to make the opengl_context current.
        // Will the dummy window be rendererd to if no other window is made current?
        // winuser::ReleaseDC(dummy_window2, dummy_window_dc2);
        // winuser::DestroyWindow(dummy_window2);

        // This dummy_window should be destroyed when the context is dropped.
        error_if_false(
            wingdi::wglMakeCurrent(dummy_window_dc2, opengl_context),
            true,
        )?;

        Ok(OpenGLContext {
            context_ptr: opengl_context,
            pixel_format_id,
            pixel_format_descriptor: pfd,
        })
    }
}

fn create_dummy_window(h_instance: HINSTANCE, class_name: &Vec<u16>) -> windef::HWND {
    let title = win32_string("Dummy");

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
// The following is a bunch of scary code hand written to load some functions needed for our OpenGL context.

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
