extern crate winapi;
mod event_loop_windows;
mod gl_context_windows;
mod keys_windows;
mod utils_windows;
mod window_manager_windows;

pub use event_loop_windows::run;
pub use window_manager_windows::{Window, WindowBuilder, WindowManager};

#[cfg(feature = "opengl_glow")]
pub extern crate glow;
