use std::io::Error;
use wasm_bindgen::JsCast;

pub struct GLContextBuilder {
    select_webgl1: bool,
    select_webgl2: bool,
}

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

        let mut context = if self.select_webgl2 {
            if let Ok(context) =
                canvas.get_context_with_context_options("webgl2", context_attributes.as_ref())
            {
                let webgl2_context = context
                    .unwrap()
                    .dyn_into::<web_sys::WebGl2RenderingContext>()
                    .unwrap();
                Ok(GLContext {
                    webgl1_context: None,
                    webgl2_context: Some(webgl2_context),
                })
            } else {
                Err(())
            }
        } else {
            Err(())
        };

        if self.select_webgl1 && context.is_err() {
            let webgl1_context = canvas
                .get_context_with_context_options("webgl", context_attributes.as_ref())
                .unwrap()
                .unwrap()
                .dyn_into::<web_sys::WebGlRenderingContext>()
                .unwrap();
            context = Ok(GLContext {
                webgl1_context: Some(webgl1_context),
                webgl2_context: None,
            })
        }

        context
    }

    /*
    pub fn webgl1(&mut self) -> &mut Self {
        self.select_webgl1 = true;
        self
    }
    */

    /// Attempts to get a WebGL2 context first, if that doesn't work
    /// fall back to WebGL1
    pub fn webgl2(&mut self) -> &mut Self {
        self.select_webgl2 = true;
        self
    }
}

pub struct GLContext {
    webgl1_context: Option<web_sys::WebGlRenderingContext>,
    webgl2_context: Option<web_sys::WebGl2RenderingContext>,
}

// This is fine because web is single threaded.
unsafe impl Send for GLContext {}

impl GLContext {
    pub fn new() -> GLContextBuilder {
        GLContextBuilder {
            select_webgl1: true,
            select_webgl2: false,
        }
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

    pub fn webgl1_context(&self) -> Option<web_sys::WebGlRenderingContext> {
        self.webgl1_context.clone()
    }

    pub fn webgl2_context(&self) -> Option<web_sys::WebGl2RenderingContext> {
        self.webgl2_context.clone()
    }

    pub fn swap_buffers(&self) {
        // Happens automatically for web, so do nothing!
    }
}
