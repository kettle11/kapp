/// A wrapper around a gl_context to fit in better with the rest of kettlewin
use kettlewin_gl_context;

pub struct GLContext {
    inner_context: kettlewin_gl_context::GLContext,
}

pub struct GLContextBuilder {}

impl GLContextBuilder {
    pub fn build(&self) -> Result<GLContext, ()> {
        kettlewin_gl_context::GLContext::new()
            .build()
            .map(|g| GLContext { inner_context: g })
    }
}

impl GLContext {
    pub fn new() -> GLContextBuilder {
        GLContextBuilder {}
    }

    pub fn set_window(&mut self, window: Option<&crate::Window>) -> Result<(), std::io::Error> {
        let raw_window = unsafe { window.map(|w| w.id.inner_window()) };
        self.inner_context.set_window(raw_window)
    }

    // Updates the target window's backbuffer if necessary.
    pub fn update_target(&self) {
        self.inner_context.update_target();
    }

    pub fn make_current(&self) {
        self.inner_context.make_current();
    }

    pub fn swap_buffers(&self) {
        self.inner_context.swap_buffers();
    }

    pub fn gl_loader_c_string(&self) -> Box<dyn FnMut(*const i8) -> *const std::ffi::c_void> {
        self.inner_context.gl_loader_c_string()
    }

    pub fn get_proc_address(&self, addr: &str) -> *const core::ffi::c_void {
        kettlewin_gl_context::GLContext::get_proc_address(addr)
    }
}
