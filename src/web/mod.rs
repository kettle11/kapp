extern crate web_sys;
mod event_loop_web;
pub use event_loop_web::run;
pub mod window_manager_web;
pub use window_manager_web::{Window, WindowBuilder, WindowManager};
