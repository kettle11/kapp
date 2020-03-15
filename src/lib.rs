mod async_application;
mod events;
mod keys;
mod mouse_buttons;

pub use async_application::*;
pub use keys::Key;
pub use events::*;
pub use mouse_buttons::MouseButton;

#[cfg(all(target_os = "windows"))]
mod windows;
#[cfg(all(target_os = "windows"))]
pub use windows::*;

#[cfg(all(target_os = "macos"))]
mod macos;
#[cfg(all(target_os = "macos"))]
pub use macos::*;

#[cfg(target_arch = "wasm32")]
mod web;
#[cfg(target_arch = "wasm32")]
pub use web::*;

#[cfg(feature = "opengl_glow")]
pub extern crate glow;

#[cfg(all(target_os = "macos"))]
#[macro_use]
extern crate objc;
