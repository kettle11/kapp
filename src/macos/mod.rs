// This could be in build.rs, but it's simpler to have to here.
// This causes AppKit to be linked.
#[link(name = "AppKit", kind = "framework")]
extern "C" {}

#[allow(non_upper_case_globals, non_snake_case)]
mod apple;

mod events_mac;
mod keys_mac;
mod window_mac;

mod gl_context;
pub use gl_context::*;
mod application_mac;

pub use application_mac::{Application, EventLoop};
pub use window_mac::{Window, WindowId};
