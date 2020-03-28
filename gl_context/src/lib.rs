mod gl_context_builder;
mod gl_context_trait;

use gl_context_builder::GLContextBuilder;

#[cfg(target_os = "macos")]
mod macos;

#[cfg(target_arch = "wasm32")]
mod web;

pub use gl_context_trait::*;

#[cfg(target_os = "macos")]
pub use macos::GLContext;

#[cfg(target_arch = "wasm32")]
pub use web::GLContext;
