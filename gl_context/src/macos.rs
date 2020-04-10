use objc::runtime::{Object, YES};
use objc::*;
use std::ffi::c_void;
use crate::common::*;
use std::io::Error;

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
    pub fn build(&self) -> Result<GLContext, ()> {
        unsafe {
            let attrs = [
                NSOpenGLPFAOpenGLProfile as u32,
                NSOpenGLProfileVersion4_1Core as u32, 
                NSOpenGLPFAColorSize as u32,
                self.gl_attributes.color_bits as u32,
                NSOpenGLPFAAlphaSize as u32,
                self.gl_attributes.alpha_bits as u32,
                NSOpenGLPFADepthSize as u32,
                self.gl_attributes.depth_bits as u32,
                NSOpenGLPFAStencilSize as u32,
                self.gl_attributes.stencil_bits as u32,
                NSOpenGLPFAAccelerated as u32,
                NSOpenGLPFADoubleBuffer as u32,
                NSOpenGLPFASampleBuffers as u32,
                1,
                NSOpenGLPFASamples as u32,
                self.gl_attributes.msaa_samples as u32,
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
        GLContextBuilder {
            gl_attributes: GLContextAttributes {
                version_major: 3,
                version_minor: 3,
                msaa_samples: 1,
                color_bits: 24,
                alpha_bits: 8,
                depth_bits: 24,
                stencil_bits: 8,
            },
        }
    }
}

impl GLContextTrait for GLContext {
    fn set_window(&mut self, window: Option<&impl raw_window_handle::HasRawWindowHandle>) -> Result<(), SetWindowError> {
        use raw_window_handle::*;

        let window = window.map(|w|  
            match w.raw_window_handle() {
            RawWindowHandle::MacOS(handle) => {
                handle.ns_window as *mut Object
            }
            _ => unreachable!()
        });

        if let Some(window) = window {
            let window_view: *mut Object = unsafe { msg_send![window, contentView] };
            let () = unsafe { msg_send![self.gl_context, setView: window_view] };
        } else {
            let () = unsafe { msg_send![self.gl_context, clearDrawable] };
        }

        Ok(())
    }

    fn get_attributes(&self) -> GLContextAttributes {
        unimplemented!()
    }

    fn set_vsync(&mut self, vsync: VSync) -> Result<(), std::io::Error> {
        unimplemented!()
    }

    fn get_vsync(&self) -> VSync { 
        unimplemented!()
    }

    fn make_current(&mut self) -> Result<(), Error>{
        unsafe {
            let () = msg_send![self.gl_context, makeCurrentContext];
        }
        Ok(())
    }

    // https://developer.apple.com/documentation/appkit/nsopenglcontext/1436211-flushbuffer?language=objc
    fn swap_buffers(&mut self) {
        unsafe {
            let () = msg_send![self.gl_context, flushBuffer];
        }
    }

    // Taken from Glutin:
    // https://github.com/rust-windowing/glutin/blob/447f3526dcf90a460d52afefd0b29eb2ed7f87f3/glutin/src/platform_impl/macos/mod.rs
    fn get_proc_address(&self, addr: &str) -> *const core::ffi::c_void {
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
