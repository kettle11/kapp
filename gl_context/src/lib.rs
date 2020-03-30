mod gl_context_trait;

#[cfg(target_os = "macos")]
mod macos;

#[cfg(target_arch = "wasm32")]
mod web;

pub use gl_context_trait::*;

#[cfg(target_os = "macos")]
pub use macos::{GLContext, GLContextBuilder};

#[cfg(target_arch = "wasm32")]
pub use web::{GLContext, GLContextBuilder};
