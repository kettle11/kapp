// This is specifically a 'web' backend and not a general 'wasm' backend
// In the future Wasm may be a more general purpose platform, and this backend
// is not appropriate for all wasm contexts.

extern crate web_sys;
mod event_loop_web;
mod keys_web;
pub use event_loop_web::run;
pub mod window_manager_web;
pub use window_manager_web::{Window, WindowBuilder, WindowManager};
