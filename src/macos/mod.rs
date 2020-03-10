// This could be in build.rs, but it's simpler to have to here.
// This causes AppKit to be linked.
#[link(name = "AppKit", kind = "framework")]
extern "C" {}

#[allow(non_upper_case_globals)]
mod apple;

mod event_loop_macos;
mod keys_mac;

mod window_manager_macos;

pub use event_loop_macos::run;
pub use window_manager_macos::{App, Window};
