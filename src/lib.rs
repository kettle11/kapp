//! Cross platform windows, input, and GL context creation for Windows, Mac, and Web.
//!
//! # Hello Window
//! ```no_run
//! use kettlewin::*;
//!
//! fn main() {
//!     // Initialize the Application and EventLoop
//!     let (app, event_loop) = initialize();
//!
//!     // Open a window
//!     let _window = app.new_window().build().unwrap();
//!
//!     // Run forever receiving system events.
//!     event_loop.run( move |event| match event {
//!          WindowCloseRequested { .. } => app.quit(),
//!          Event::Draw { .. } => {
//!            // Render something here.
//!          }
//!          _ => println!("Received event: {:?}", event),
//!     });
//! }
//! ```
//!
//! # User Input
//! Events are provided for user input:
//!
//! [KeyDown][Event::KeyDown], [KeyUp][Event::KeyUp], [MouseMoved][Event::MouseMoved],
//! [MouseButtonDown][Event::MouseButtonDown], [MouseButtonUp][Event::MouseButtonUp], [ScrollWheel][Event::ScrollWheel]
//!
//! If an event responds with coordinates the coordinates are in physical device space
//! (the actual pixels of the device without a scale factor applied).
//! The origin (0,0) is the upper left corner of the screen or window.
//! ```no_run
//! use kettlewin::*;
//!
//! fn main() {
//!     let (mut app, event_loop) = initialize();
//!     let _window = app.new_window().build().unwrap();
//!
//!     event_loop.run( move |event| match event {
//!         Event::KeyDown { key, .. } => println!("Key pressed: {:?}", key),
//!         Event::KeyUp { key, .. } => println!("Key up: {:?}", key),
//!         Event::MouseMoved { x, y, .. } => println!("Mouse moved: {:?},{:?}", x, y),
//!         _ => {},
//!     });
//! }
//! ```
//!
//! # GL Rendering
//! If the `gl_context` feature is enabled then a GLContext can be created for rendering with GL.
//! See the `simple_gl.rs` example.
mod application;
mod async_application;
mod state_tracker;
mod window;
mod window_builder;

#[cfg(target_os = "macos")]
use kettlewin_platform_macos::prelude as platform;

#[cfg(target_arch = "wasm32")]
use kettlewin_platform_web::prelude as platform;

#[cfg(target_os = "windows")]
use kettlewin_platform_windows::prelude as platform;

#[cfg(feature = "gl_context")]
pub use kettlewin_gl_context::prelude::*;

pub use platform::{Cursor, Event, Key, MouseButton, WindowId};

pub use application::{initialize, Application, EventLoop};

pub use async_application::*;

pub use window::Window;
