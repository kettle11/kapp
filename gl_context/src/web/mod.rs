use crate::common::*;
use kwasm::*;

static mut KAPP_GL_MODULE: KWasmModule = KWasmModule::null();

#[repr(u32)]
enum HostMessage {
    CreateWebGL1Context = 0,
    CreateWebGL2Context = 1,
}

impl GLContextBuilder {
    pub fn build(&self) -> Result<GLContext, ()> {
        unsafe {
            if KAPP_GL_MODULE.is_null() {
                KAPP_GL_MODULE = kwasm::register_module(include_str!("kapp_gl_module.js"));
            }

            match self.gl_attributes.webgl_version {
                WebGLVersion::One => {
                    kwasm::send_message_to_host(
                        KAPP_GL_MODULE,
                        HostMessage::CreateWebGL1Context as u32,
                    );
                }
                WebGLVersion::Two => {
                    kwasm::send_message_to_host(
                        KAPP_GL_MODULE,
                        HostMessage::CreateWebGL2Context as u32,
                    );
                }
                WebGLVersion::None => Err(())?,
            }
        }
        Ok(GLContext {})
    }

    pub fn webgl1(&mut self) -> &mut Self {
        self.gl_attributes.webgl_version = WebGLVersion::One;
        self
    }

    /// Attempts to get a WebGL2 context first, if that doesn't work
    /// fall back to WebGL1
    pub fn webgl2(&mut self) -> &mut Self {
        self.gl_attributes.webgl_version = WebGLVersion::Two;
        self
    }
}

pub struct GLContext {}

impl GLContext {
    pub fn new() -> GLContextBuilder {
        GLContextBuilder {
            gl_attributes: GLContextAttributes {
                // None of these attributes other than webgl_version are used.
                version_major: 3,
                version_minor: 3,
                msaa_samples: 1,
                color_bits: 24,
                alpha_bits: 8,
                depth_bits: 24,
                stencil_bits: 8,
                webgl_version: WebGLVersion::One,
                high_resolution_framebuffer: false,
            },
        }
    }
}

impl GLContextTrait for GLContext {
    fn get_attributes(&self) -> GLContextAttributes {
        unimplemented!()
    }

    fn set_vsync(&mut self, _vsync: VSync) -> Result<(), std::io::Error> {
        Ok(()) // Unsupported on web
    }

    fn get_vsync(&self) -> VSync {
        VSync::On
    }

    fn resize(&mut self) {
        // Do nothing
    }

    fn get_proc_address(&self, _address: &str) -> *const core::ffi::c_void {
        unimplemented!() // Should not be called for web
    }

    fn set_window(
        &mut self,
        _window: Option<&impl raw_window_handle::HasRawWindowHandle>,
    ) -> Result<(), SetWindowError> {
        Ok(()) // Does nothing on web
    }

    fn make_current(&mut self) -> Result<(), std::io::Error> {
        Ok(()) // Does nothing on web
    }

    fn swap_buffers(&mut self) {
        // Happens automatically for web, so do nothing!
    }
}
