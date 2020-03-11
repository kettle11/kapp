// This could be in build.rs, but it's simpler to have to here.
// This causes AppKit to be linked.
#[link(name = "AppKit", kind = "framework")]
extern "C" {}

#[allow(non_upper_case_globals)]
mod apple;

mod events_mac;
mod keys_mac;

mod gl_context;
pub use gl_context::*;
mod window_manager_macos;

pub use window_manager_macos::{App, Window};

mod async_application;
pub use async_application::{AsyncApplication, Events};
