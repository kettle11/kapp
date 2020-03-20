/// This is specifically a 'web' backend and not a general 'wasm' backend
/// In the future Wasm may be a more general purpose platform, and this backend
/// is not appropriate for all wasm contexts.
extern crate web_sys;
mod event_loop_web;
mod gl_context_web;
mod keys_web;
pub use event_loop_web::run;
pub use gl_context_web::GLContext;
pub mod application_web;
pub use application_web::{PlatformApplication, PlatformChannel, PlatformWaker};

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
pub struct WindowId {}
