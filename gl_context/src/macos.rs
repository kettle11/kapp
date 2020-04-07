use objc::runtime::{Object, YES};
use objc::*;
use std::ffi::c_void;
use std::io::Error;

pub struct GLContextBuilder {
    samples: u32,
}

pub struct GLContext {
    gl_context: *mut Object,
    pixel_format: *mut Object,
    // current_window: Option<*mut Object>,
}

// This isn't really true because make_current must be called after GLContext is passed to another thread.
// A solution would be to wrap this is an object to send to another thread, and the unwrap
// calls make_current.
unsafe impl Send for GLContext {}

impl GLContextBuilder {
    pub fn samples(&mut self, samples: u32) -> &mut Self {
        self.samples = samples;
        self
    }

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
                NSOpenGLPFASampleBuffers as u32,
                1,
                NSOpenGLPFASamples as u32,
                self.samples as u32,
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
                // current_window: None,
            })
        }
    }
}

impl GLContext {
    pub fn new() -> GLContextBuilder {
        GLContextBuilder { samples: 2 }
    }

    pub fn set_window(
        &mut self,
        window: Option<&kettlewin_platform_common::WindowId>,
    ) -> Result<(), Error> {
        let window = window.map(|w| unsafe { w.raw() } as *mut std::ffi::c_void);
        self.set_window_raw(window)
    }

    pub fn set_window_raw(&mut self, window: Option<*mut std::ffi::c_void>) -> Result<(), Error> {
        let window = window.map(|w| w as *mut Object);
        if let Some(window) = window {
            let window_view: *mut Object = unsafe { msg_send![window, contentView] };

            let () = unsafe { msg_send![self.gl_context, setView: window_view] };
        // self.current_window = Some(window.clone());
        } else {
            let () = unsafe { msg_send![self.gl_context, clearDrawable] };
            //  self.current_window = None;
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

    pub fn gl_loader_c_string(&self) -> Box<dyn FnMut(*const i8) -> *const std::ffi::c_void> {
        Box::new(move |s| unsafe {
            let name = std::ffi::CStr::from_ptr(s);
            Self::get_proc_address_inner((&name).to_str().unwrap())
        })
    }

    // https://developer.apple.com/documentation/appkit/nsopenglcontext/1436211-flushbuffer?language=objc
    pub fn swap_buffers(&self) {
        unsafe {
            let () = msg_send![self.gl_context, flushBuffer];
        }
    }

    pub fn get_proc_address(&self, addr: &str) -> *const core::ffi::c_void {
        Self::get_proc_address_inner(addr)
    }

    // Taken from Glutin:
    // https://github.com/rust-windowing/glutin/blob/447f3526dcf90a460d52afefd0b29eb2ed7f87f3/glutin/src/platform_impl/macos/mod.rs
    fn get_proc_address_inner(addr: &str) -> *const core::ffi::c_void {
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

// These enums are taken from the core-foundation-rs crate
#[repr(u64)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NSOpenGLContextParameter {
    NSOpenGLCPSwapInterval = 222,
}
pub use NSOpenGLContextParameter::*;

#[repr(u64)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NSOpenGLPixelFormatAttribute {
    NSOpenGLPFADoubleBuffer = 5,
    NSOpenGLPFAColorSize = 8,

    NSOpenGLPFAAlphaSize = 11,
    NSOpenGLPFADepthSize = 12,
    NSOpenGLPFAStencilSize = 13,
    NSOpenGLPFASampleBuffers = 55,
    NSOpenGLPFASamples = 56,
    NSOpenGLPFAAccelerated = 73,
    NSOpenGLPFAOpenGLProfile = 99,
}
pub use NSOpenGLPixelFormatAttribute::*;

#[repr(u64)]
#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NSOpenGLPFAOpenGLProfiles {
    //NSOpenGLProfileVersion3_2Core = 0x3200,
    NSOpenGLProfileVersion4_1Core = 0x4100,
}
pub use NSOpenGLPFAOpenGLProfiles::*;
pub struct NSString {
    pub raw: *mut Object,
}

impl NSString {
    pub fn new(string: &str) -> Self {
        unsafe {
            let raw: *mut Object = msg_send![class!(NSString), alloc];
            let raw: *mut Object = msg_send![
                raw,
                initWithBytes: string.as_ptr()
                length: string.len()
                encoding:UTF8_ENCODING as *mut Object
            ];

            Self { raw }
        }
    }
}

impl Drop for NSString {
    fn drop(&mut self) {
        unsafe {
            let () = msg_send![self.raw, release];
        }
    }
}

#[allow(non_upper_case_globals)]
pub const nil: *mut Object = 0 as *mut Object;

#[repr(C)]
pub struct __CFBundle(c_void);
pub type CFBundleRef = *mut __CFBundle;

extern "C" {
    pub fn CFBundleGetBundleWithIdentifier(bundleID: CFStringRef) -> CFBundleRef;
    pub fn CFBundleGetFunctionPointerForName(
        bundle: CFBundleRef,
        function_name: CFStringRef,
    ) -> *const c_void;
}

pub const UTF8_ENCODING: usize = 4;
pub type CFStringRef = *const Object; // CFString
