use crate::common::*;
use wasm_bindgen::JsCast;

impl GLContextBuilder {
    pub fn build(&self) -> Result<GLContext, ()> {
        let canvas = web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .get_element_by_id("canvas")
            .unwrap()
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .unwrap();

        // These should be configurable
        let mut context_attributes = web_sys::WebGlContextAttributes::new();
        context_attributes.alpha(false); // Disable the canvas background transparency.

        let context = match self.gl_attributes.webgl_version {
            WebGLVersion::One => {
                let webgl1_context = canvas
                    .get_context_with_context_options("webgl", context_attributes.as_ref())
                    .unwrap()
                    .unwrap()
                    .dyn_into::<web_sys::WebGlRenderingContext>()
                    .unwrap();
                Ok(GLContext {
                    webgl1_context: Some(webgl1_context),
                    webgl2_context: None,
                })
            }
            WebGLVersion::Two => {
                let webgl2_context = canvas
                    .get_context_with_context_options("webgl2", context_attributes.as_ref())
                    .unwrap()
                    .unwrap()
                    .dyn_into::<web_sys::WebGl2RenderingContext>()
                    .unwrap();
                Ok(GLContext {
                    webgl1_context: None,
                    webgl2_context: Some(webgl2_context),
                })
            }
            WebGLVersion::None => Err(()),
        };

        context
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

pub struct GLContext {
    webgl1_context: Option<web_sys::WebGlRenderingContext>,
    webgl2_context: Option<web_sys::WebGl2RenderingContext>,
}

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
                high_dpi_framebuffer: false,
            },
        }
    }

    pub fn webgl1_context(&self) -> Option<web_sys::WebGlRenderingContext> {
        self.webgl1_context.clone()
    }

    pub fn webgl2_context(&self) -> Option<web_sys::WebGl2RenderingContext> {
        self.webgl2_context.clone()
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
