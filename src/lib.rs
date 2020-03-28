//! Cross platform windows, input, and GL context creation.
//!
//! # Hello Window
//! ```no_run
//! use kettlewin::*;
//!
//! fn main() {
//!     // Initialize the Application and EventLoop
//!     let (mut app, event_loop) = initialize();
//!
//!     // Open a window
//!     let _window = app.new_window().build().unwrap();
//!
//!     // Run forever receiving system events.
//!     event_loop.run( move |event| match event {
//!          WindowCloseRequested { .. } => app.quit(),
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
//! The origin (0,0) is the lower left corner of the screen or window.
//! ```no_run
//! use kettlewin::*;
//!
//! fn main() {
//!     let (mut app, event_loop) = initialize();
//!     let _window = app.new_window().build().unwrap();
//!
//!     event_loop.run( move |event| match event {
//!         Event::KeyDown { key } => println!("Key pressed: {:?}", key),
//!         Event::KeyUp { key } => println!("Key up: {:?}", key),
//!         Event::MouseMoved { x, y } => println!("Mouse moved: {:?},{:?}", x, y),
//!         _ => {},
//!     });
//! }
//! ```
//!
//! # GL Rendering
//! If the `gl_context` feature is enabled then a GLContext can be created for rendering with GL.
//! ```no_run
//! use kettlewin::*;
//! use glow::*;
//!
//! fn main() {
//!     let (mut app, event_loop) = initialize();
//!     let window = app.new_window().build().unwrap();
//!     
//!     // Create a GLContext
//!     let mut gl_context = GLContext::new().build().unwrap();
//!
//!     // Assign the GLContext's window.
//!     gl_context.set_window(Some(&window.id)).unwrap();
//!     
//!     // Glow is a library for accessing GL function calls from a variety of platforms
//!     // Glow requires a cross platform way to load function pointers,
//!     // which GLContext provides with get_proc_address.
//!     let gl = glow::Context::from_loader_function(|s| gl_context.get_proc_address(s));
//!
//!     event_loop.run( move |event| match event {
//!        Event::Draw => {
//!             // Make the GLContext current to the thread that this callback runs on.
//!             gl_context.make_current();
//!
//!             // Clear the screen to a lovely shade of blue.
//!             unsafe {
//!                 gl.clear_color(0.3765, 0.3137, 0.8627, 1.0);
//!                 gl.clear(COLOR_BUFFER_BIT | DEPTH_BUFFER_BIT);
//!             }
//!
//!             // Finally display what we've drawn.
//!             gl_context.swap_buffers();
//!
//!             // It is not necessary for this example,
//!             // but calling request_frame ensures the program redraws continuously.
//!             app.request_frame();
//!        }
//!         _ => {},
//!     });
//! }
//! ```
mod application;
mod window;
mod window_builder;

#[cfg(target_os = "windows")]
mod windows;

#[cfg(target_os = "macos")]
use kettlewin_platform_macos::prelude as platform;

#[cfg(target_arch = "wasm32")]
use kettlewin_platform_web::prelude as platform;

#[cfg(feature = "gl_context")]
pub use kettlewin_gl_context::GLContext;

#[cfg(target_os = "windows")]
pub use windows::*;

pub use platform::{Event, Key, MouseButton, WindowId};

pub use application::{initialize, Application, EventLoop};

//pub use async_application::*;

pub use window::Window;
