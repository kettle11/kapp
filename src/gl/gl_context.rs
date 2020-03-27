/// A wrapper around a gl_context to fit in better with the rest of kettlewin
use kettlewin_gl_context;

/// Create a GLContext and attach it to a window before using any GL commands.
pub struct GLContext {
    inner_context: kettlewin_gl_context::GLContext,
}

// GLContext holds a raw pointer to its assigned window,
// which must be used carefully.
// However the raw pointer is not exposed. 
unsafe impl Send for GLContext {}

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

    /// Attaches a window to this GLContext.
    pub fn set_window(&mut self, window: Option<&crate::Window>) -> Result<(), std::io::Error> {
        let raw_window = unsafe { window.map(|w| w.id.raw() as *mut std::ffi::c_void) };
        self.inner_context.set_window(raw_window)
    }

    /// Updates the attached window's backbuffer scale.
    pub fn update_target(&self) {
        self.inner_context.update_target();
    }

    /// One GL context at a time is associated with a thread.
    /// 'make_current' associates the current GLContext with the current thread.
    /// 'make_current' must be called before using GL on a thread.
    /// A GLContext is automatically set to current for the thread it is created on.
    pub fn make_current(&self) {
        self.inner_context.make_current();
    }

    /// Swaps the attached window's backbuffers.
    /// Call this to display rendered visuals to the screen.
    pub fn swap_buffers(&self) {
        self.inner_context.swap_buffers();
    }

    /// Returns a function that accepts a C string and returns a raw function pointer.
    /// Used by some libraries for loading GL functions.
    pub fn gl_loader_c_string(&self) -> Box<dyn FnMut(*const i8) -> *const std::ffi::c_void> {
        self.inner_context.gl_loader_c_string()
    }

    /// Accepts a function address and returns a raw function pointer.
    /// Used by GL loaders like Glow to load GL functions.
    pub fn get_proc_address(&self, addr: &str) -> *const core::ffi::c_void {
        kettlewin_gl_context::GLContext::get_proc_address(addr)
    }
}
