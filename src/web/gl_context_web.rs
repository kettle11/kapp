use crate::Window;
use std::io::Error;
use wasm_bindgen::JsCast;

pub struct GLContext {
    webgl_context: web_sys::WebGlRenderingContext,
}

// This is fine because web is single threaded.
unsafe impl Send for GLContext {}

pub struct GLContextBuilder {}

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

        // There should be a way to choose webgl1 or webgl2
        // Or perhaps choose automatically based on support?
        /*
        if let Some(canvas) = canvas.get_context("webgl2").unwrap() {
            let webgl2_context = canvas
                .dyn_into::<web_sys::WebGl2RenderingContext>()
                .unwrap();
            glow::Context::from_webgl2_context(webgl2_context)
        } else {*/
        let webgl_context = canvas
            .get_context_with_context_options("webgl", context_attributes.as_ref())
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::WebGlRenderingContext>()
            .unwrap();
        Ok(GLContext { webgl_context })
    }
}

impl GLContext {
    pub fn new() -> GLContextBuilder {
        GLContextBuilder {}
    }

    pub fn set_window(&self, _window: Option<&Window>) -> Result<(), Error> {
        Ok(())
    }

    pub fn update_target(&self) {}
    pub fn make_current(&self) {}

    #[cfg(feature = "opengl_glow")]
    pub fn glow_context(&self) -> glow::Context {
        glow::Context::from_webgl1_context(self.webgl_context.clone())
    }

    pub fn swap_buffers(&self) {
        // Happens automatically for web, so do nothing!
    }
}
