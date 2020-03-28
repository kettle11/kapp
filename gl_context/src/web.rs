use crate::GLContextBuilder;
use std::io::Error;
use wasm_bindgen::JsCast;

pub struct GLContext {
    webgl_context: web_sys::WebGlRenderingContext,
}

// This is fine because web is single threaded.
unsafe impl Send for GLContext {}

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

    pub fn set_window(
        &mut self,
        window: Option<&kettlewin_platform_common::WindowId>,
    ) -> Result<(), Error> {
        let window = window.map(|w| unsafe { w.raw() } as *mut std::ffi::c_void);
        self.set_window_raw(window)
    }

    pub fn set_window_raw(&self, _window: Option<*mut std::ffi::c_void>) -> Result<(), Error> {
        Ok(())
    }

    pub fn update_target(&self) {}
    pub fn make_current(&self) {}

    pub fn get_webgl1_context(&self) -> web_sys::WebGlRenderingContext {
        self.webgl_context.clone()
    }

    pub fn swap_buffers(&self) {
        // Happens automatically for web, so do nothing!
    }
}
