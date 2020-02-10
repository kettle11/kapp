use std::io::Error;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::console;

pub struct Window {}

pub struct WindowBuilder<'a> {
    x: Option<u32>,
    y: Option<u32>,
    width: Option<u32>,
    height: Option<u32>,
    resizable: bool,
    title: Option<&'a str>,
}

impl<'a> WindowBuilder<'a> {
    pub fn title(&mut self, title: &'a str) -> &mut Self {
        self.title = Some(title);
        self
    }

    pub fn position(&mut self, x: u32, y: u32) -> &mut Self {
        self.x = Some(x);
        self.y = Some(y);
        self
    }
    pub fn dimensions(&mut self, width: u32, height: u32) -> &mut Self {
        self.width = Some(width);
        self.height = Some(height);
        self
    }

    pub fn build(&self) -> Result<Window, Error> {
        Ok(Window {})
    }
}

pub struct WindowManagerBuilder {
    color_bits: u8,
    alpha_bits: u8,
    depth_bits: u8,
    stencil_bits: u8,
    samples: u8,
    srgb: bool,
}

impl WindowManagerBuilder {
    pub fn bits(
        &mut self,
        color_bits: u8,
        alpha_bits: u8,
        depth_bits: u8,
        stencil_bits: u8,
    ) -> &mut Self {
        self.color_bits = color_bits;
        self.alpha_bits = alpha_bits;
        self.depth_bits = depth_bits;
        self.stencil_bits = stencil_bits;
        self
    }
    pub fn color_bits(&mut self, bits: u8) -> &mut Self {
        self.color_bits = bits;
        self
    }

    pub fn alpha_bits(&mut self, bits: u8) -> &mut Self {
        self.alpha_bits = bits;
        self
    }

    pub fn depth_bits(&mut self, bits: u8) -> &mut Self {
        self.depth_bits = bits;
        self
    }

    pub fn stencil_bits(&mut self, bits: u8) -> &mut Self {
        self.stencil_bits = bits;
        self
    }

    /// Sets the MSAA samples.
    /// Set this to a power of 2.
    /// With an Nvidia card on Windows I was unable to set this below 2.
    pub fn samples(&mut self, samples: u8) -> &mut Self {
        self.samples = samples;
        self
    }

    /// This sets if the backbuffer for the windows will be in sRGB color space... or it would if drivers respected it.
    /// Unfortunately this flag does nothing as tested on Windows with an Nvidia GPU.
    /// In that case backbuffer was set to sRGB colorspace.
    pub fn srgb(&mut self, srgb: bool) -> &mut Self {
        self.srgb = srgb;
        self
    }

    pub fn build(&self) -> Result<WindowManager, Error> {
        Ok(WindowManager {})
    }
}
pub struct WindowManager {}

impl WindowManager {
    pub fn new() -> WindowManagerBuilder {
        WindowManagerBuilder {
            color_bits: 32,
            alpha_bits: 8,
            depth_bits: 16,
            stencil_bits: 0,
            samples: 1,
            srgb: true,
        }
    }

    pub fn new_window<'a>(&mut self) -> WindowBuilder<'a> {
        WindowBuilder {
            x: None,
            y: None,
            width: None,
            height: None,
            resizable: true,
            title: None,
        }
    }

    pub fn swap_buffers(&self, _window: &Window) {}

    #[cfg(feature = "opengl_glow")]
    pub fn gl_context(&self) -> glow::Context {
        let canvas = web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .get_element_by_id("canvas")
            .unwrap()
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .unwrap();
        // There should be a way to choose webgl1 or webgl2
        // Or perhaps choose automatically based on support?

        if let Some(canvas) = canvas.get_context("webgl2").unwrap() {
            let webgl2_context = canvas
                .dyn_into::<web_sys::WebGl2RenderingContext>()
                .unwrap();
            glow::Context::from_webgl2_context(webgl2_context)
        } else {
            let webgl_context = canvas
                .get_context("webgl")
                .unwrap()
                .unwrap()
                .dyn_into::<web_sys::WebGlRenderingContext>()
                .unwrap();
            glow::Context::from_webgl1_context(webgl_context)
        }
    }
}
