use crate::common::*;
use std::io::Error;

pub struct GLContext {}

// This isn't really true because make_current must be called after GLContext is passed to another thread.
// A solution would be to wrap this is an object to send to another thread, and the unwrap
// calls make_current.
unsafe impl Send for GLContext {}

impl GLContextBuilder {
    pub fn build(&self) -> Result<GLContext, ()> {
        unimplemented!()
    }
}

impl GLContext {
    pub fn new() -> GLContextBuilder {
        GLContextBuilder {
            gl_attributes: GLContextAttributes {
                major_version: 3,
                minor_version: 3,
                msaa_samples: 1,
                color_bits: 24,
                alpha_bits: 8,
                depth_bits: 24,
                stencil_bits: 8,
                srgb: true,
                webgl_version: WebGLVersion::None,
            },
        }
    }
}

impl GLContextTrait for GLContext {
    fn set_window(
        &mut self,
        window: Option<&impl raw_window_handle::HasRawWindowHandle>,
    ) -> Result<(), SetWindowError> {
        unimplemented!()
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

    fn make_current(&mut self) -> Result<(), Error> {
        unimplemented!()
    }

    fn resize(&mut self) {
        unimplemented!()
    }

    // https://developer.apple.com/documentation/appkit/nsopenglcontext/1436211-flushbuffer?language=objc
    fn swap_buffers(&mut self) {
        unimplemented!()
    }

    fn get_proc_address(&self, addr: &str) -> *const core::ffi::c_void {
        unimplemented!()
    }
}

impl Drop for GLContext {
    fn drop(&mut self) {
        unimplemented!()
    }
}
