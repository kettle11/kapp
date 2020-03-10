#[non_exhaustive]
pub struct AppParameters {
    color_bits: u8,
    alpha_bits: u8,
    depth_bits: u8,
    stencil_bits: u8,
    samples: u8,
    srgb: bool,
}

impl Default for AppParameters {
    fn default() -> Self {
        Self {
            color_bits: 32,
            alpha_bits: 8,
            depth_bits: 16,
            stencil_bits: 0,
            samples: 1,
            srgb: true,
        }
    }
}
