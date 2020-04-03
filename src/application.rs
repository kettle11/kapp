use crate::platform::*;
use crate::platform::{PlatformApplicationTrait, PlatformEventLoopTrait};
use std::cell::RefCell;
use std::rc::Rc;

/// A handle used to do things like quit,
/// request a new frame, or create windows.
#[derive(Clone)]
pub struct Application {
    pub(crate) platform_application: Rc<RefCell<PlatformApplication>>,
}

/// Create an Application and EventLoop.
pub fn initialize() -> (Application, EventLoop) {
    let platform_application = Rc::new(RefCell::new(PlatformApplication::new()));
    let platform_event_loop = platform_application.borrow_mut().event_loop();
    (
        Application {
            platform_application: platform_application.clone(),
        },
        EventLoop {
            platform_event_loop,
        },
    )
}

impl Application {
    /// Returns a new window builder.
    /// Call .build() on the window builder to complete the creation of the window.
    /// See [`crate::window_builder::WindowBuilder`] for more ways to setup a window.
    pub fn new_window(&mut self) -> crate::window_builder::WindowBuilder {
        crate::window_builder::WindowBuilder::new(self)
    }

    /// Immediately quits the application.
    pub fn quit(&mut self) {
        self.platform_application.borrow_mut().quit();
    }

    /// Sets the mouse position relative to the screen.
    /// Coordinates are expressed in physical coordinates.
    pub fn set_mouse_position(&mut self, x: u32, y: u32) {
        self.platform_application
            .borrow_mut()
            .set_mouse_position(x, y);
    }
}

// When the application is dropped, quit the program.
impl Drop for Application {
    fn drop(&mut self) {
        self.quit();
    }
}

/// Call the 'run' function on an EventLoop instance to start your program.
pub struct EventLoop {
    platform_event_loop: PlatformEventLoop,
}

impl EventLoop {
    /// Run the application forever. When a new event occurs the callback passed in will be called.
    pub fn run<T>(&mut self, callback: T)
    where
        T: 'static + FnMut(Event),
    {
        self.platform_event_loop.run(Box::new(callback));
    }
}
