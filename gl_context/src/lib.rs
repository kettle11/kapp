#[cfg(target_os = "macos")]
mod macos;

#[cfg(target_os = "windows")]
mod windows;

#[cfg(target_arch = "wasm32")]
mod web;

#[cfg(target_os = "macos")]
pub use macos::{GLContext, GLContextBuilder};

#[cfg(target_os = "windows")]
pub use windows::{GLContext, GLContextBuilder};

#[cfg(target_arch = "wasm32")]
pub use web::{GLContext, GLContextBuilder};

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum VSync {
    On,
    Off,
    Adaptive,
    Other(i32),
}
