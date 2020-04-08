pub struct GLContextAttributes {
    pub version_major: u8,
    pub version_minor: u8,
    pub color_bits: u8,
    pub alpha_bits: u8,
    pub depth_bits: u8,
    pub stencil_bits: u8,
    /// msaa_samples hould be a multiple of 2
    pub msaa_samples: u8,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum VSync {
    ///
    On,
    Off,
    Adaptive,
    /// Other will indicate how many frames to wait before displaying.
    /// For example, Other(2) would render at half the display framerate.
    Other(i32),
}

pub trait WindowTrait {
    fn raw_handle(&self) -> *mut std::ffi::c_void;
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum SetWindowError {
    /// The pixel format of the window does not match the context's
    MismatchedPixelFormat,
}

pub trait GLContextTrait {
    /// Gets the pixel format and attributes of the context.
    fn get_attributes(&self) -> GLContextAttributes;

    /// Makes the GLContext current to the current thread
    fn make_current(&mut self) -> Result<(), std::io::Error>;

    /// Sets the Vsync for the window attached to this context.
    /// Returns a system error if not successful
    fn set_vsync(&mut self, vsync: VSync) -> Result<(), std::io::Error>;
    fn get_vsync(&self) -> VSync;

    /// Asssigns a window to draw to
    fn set_window(&mut self, window: Option<&impl WindowTrait>) -> Result<(), SetWindowError>;

    /// Swaps the backbuffer and frontbuffer for the currently bound window.
    fn swap_buffers(&mut self);

    /// Gets the address of a GL process.
    /// Used by GL loaders
    fn get_proc_address(&self, address: &str) -> *const core::ffi::c_void;
}

pub struct GLContextBuilder {
    pub(crate) gl_attributes: GLContextAttributes,
}

impl GLContextBuilder {
    pub fn samples(&mut self, samples: u8) -> &mut Self {
        self.gl_attributes.msaa_samples = samples;
        self
    }
}
