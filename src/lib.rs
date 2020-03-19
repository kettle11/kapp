//mod async_application;
mod application;
mod application_message;
mod events;
mod keys;
mod mouse_buttons;
mod window;
mod window_builder;

//pub use async_application::*;
pub use application::Application;
pub use events::*;
pub use keys::Key;
pub use mouse_buttons::MouseButton;
pub use window::Window;

#[cfg(all(target_os = "windows"))]
mod windows;
#[cfg(all(target_os = "windows"))]
pub use windows::*;

#[cfg(all(target_os = "macos"))]
mod macos;
#[cfg(all(target_os = "macos"))]
pub use macos::{GLContext, PlatformApplication, WindowId};

#[cfg(target_arch = "wasm32")]
mod web;
#[cfg(target_arch = "wasm32")]
pub use web::*;

#[cfg(feature = "opengl_glow")]
pub extern crate glow;

#[cfg(all(target_os = "macos"))]
#[macro_use]
extern crate objc;
