use super::apple::*;
use super::Window;
use std::io::Error;

pub struct GLContext {
    gl_context: *mut Object,
    pixel_format: *mut Object,
    current_window: Option<Window>,
}

// This is not ok because Window is not thread safe.
unsafe impl Send for GLContext {}

pub struct GLContextBuilder {}

impl GLContextBuilder {
    pub fn build(&self) -> Result<GLContext, ()> {
        unsafe {
            let attrs = [
                NSOpenGLPFAOpenGLProfile as u32,
                NSOpenGLProfileVersion4_1Core as u32, // Needed if using opengl 3.2 you can comment this line out to use the old version.
                NSOpenGLPFAColorSize as u32,
                24,
                NSOpenGLPFAAlphaSize as u32,
                8,
                NSOpenGLPFADepthSize as u32,
                24,
                NSOpenGLPFAStencilSize as u32,
                8,
                NSOpenGLPFAAccelerated as u32,
                NSOpenGLPFADoubleBuffer as u32,
                0,
            ];

            // This allocation is dropped when GLContext is dropped
            let pixel_format: *mut Object = msg_send![class!(NSOpenGLPixelFormat), alloc];
            let pixel_format: *mut Object = msg_send![pixel_format, initWithAttributes: &attrs];

            // This allocation is dropped when GLContext is dropped
            let gl_context: *mut Object = msg_send![class!(NSOpenGLContext), alloc];
            let gl_context: *mut Object =
                msg_send![gl_context, initWithFormat: pixel_format shareContext: nil];
            let () = msg_send![gl_context, makeCurrentContext];

            // Enable vsync
            let () = msg_send![gl_context, setValues:&(1 as i32) forParameter:NSOpenGLContextParameter::NSOpenGLCPSwapInterval];
            Ok(GLContext {
                gl_context,
                pixel_format,
                current_window: None,
            })
        }
    }
}

impl GLContext {
    pub fn new() -> GLContextBuilder {
        GLContextBuilder {}
    }

    pub fn set_window(&mut self, window: Option<&Window>) -> Result<(), Error> {
        if let Some(window) = window {
            let () = unsafe {
                msg_send![self.gl_context, setView: window.inner_window_data.borrow().ns_view]
            };
            self.current_window = Some(window.clone());
        } else {
            let () = unsafe { msg_send![self.gl_context, clearDrawable] };
            self.current_window = None;
        }

        Ok(())
    }

    // Updates the backbuffer of the target when it resizes
    pub fn update_target(&self) {
        unsafe {
            let update = sel!(update);
            let () = msg_send![self.gl_context, performSelectorOnMainThread:update withObject:nil waitUntilDone:YES];
        }
    }

    pub fn make_current(&self) {
        unsafe {
            let () = msg_send![self.gl_context, makeCurrentContext];
        }
    }

    #[cfg(feature = "opengl_glow")]
    pub fn glow_context(&self) -> glow::Context {
        glow::Context::from_loader_function(|s| Self::get_proc_address(s))
    }

    pub fn gl_loader_c_string(&self) -> Box<dyn FnMut(*const i8) -> *const std::ffi::c_void> {
        Box::new(move |s| unsafe {
            let name = std::ffi::CStr::from_ptr(s);
            Self::get_proc_address((&name).to_str().unwrap())
        })
    }

    // https://developer.apple.com/documentation/appkit/nsopenglcontext/1436211-flushbuffer?language=objc
    pub fn swap_buffers(&self) {
        unsafe {
            let () = msg_send![self.gl_context, flushBuffer];
        }
    }

    // Taken from Glutin:
    // https://github.com/rust-windowing/glutin/blob/447f3526dcf90a460d52afefd0b29eb2ed7f87f3/glutin/src/platform_impl/macos/mod.rs
    fn get_proc_address(addr: &str) -> *const core::ffi::c_void {
        let symbol_name = NSString::new(addr);
        let framework_name = NSString::new("com.apple.opengl");
        let framework = unsafe { CFBundleGetBundleWithIdentifier(framework_name.raw) };
        let symbol = unsafe { CFBundleGetFunctionPointerForName(framework, symbol_name.raw) };
        symbol as *const _
    }
}

impl Drop for GLContext {
    fn drop(&mut self) {
        unsafe {
            let () = msg_send![self.gl_context, release];
            let () = msg_send![self.pixel_format, release];
        }
    }
}
