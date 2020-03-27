mod application;
//mod async_application;

mod window;
mod window_builder;

pub use application::{initialize, Application, EventLoop};
//pub use async_application::*;

pub use window::Window;

#[cfg(target_os = "windows")]
mod windows;
#[cfg(target_os = "windows")]
pub use windows::*;

#[cfg(target_arch = "wasm32")]
mod web;
#[cfg(target_arch = "wasm32")]
pub use web::*;

#[cfg(target_os = "macos")]
use kettlewin_platform_macos::prelude as platform;

pub use platform::{events::*, Event};

#[cfg(feature = "gl_context")]
mod gl;

#[cfg(feature = "gl_context")]
pub use gl::GLContext;
