mod buttons;
mod events;
pub use buttons::Button;
pub use events::*;

#[cfg(all(target_os = "windows"))]
mod windows;
#[cfg(all(target_os = "windows"))]
pub use windows::*;

#[cfg(target_arch = "wasm32")]
mod web;
#[cfg(target_arch = "wasm32")]
pub use web::*;

#[cfg(feature = "opengl_glow")]
pub extern crate glow;
