use crate::platform::*;

/// A handle used to do things like quit,
/// request a new frame, or create windows.
#[derive(Clone)]
pub struct Application {
    pub(crate) platform_channel: PlatformChannel,
    pub(crate) application_waker: PlatformWaker,
}

/// Create an Application and EventLoop.
pub fn initialize() -> (Application, EventLoop) {
    let (platform_channel, platform_application) = PlatformApplication::new();
    let application_waker = platform_application.get_waker();
    (
        Application {
            platform_channel,
            application_waker,
        },
        EventLoop {
            platform_application,
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
        self.platform_channel.send(ApplicationMessage::Quit);
        self.flush_events();
    }

    /// Sets the mouse position relative to the screen.
    /// Coordinates are expressed in physical coordinates.
    pub fn set_mouse_position(&mut self, x: u32, y: u32) {
        self.platform_channel
            .send(ApplicationMessage::SetMousePosition { x, y });
    }

    /// Blocks until the application has processed all messages sent to it.
    pub fn flush_events(&mut self) {
        self.application_waker.flush();
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
    platform_application: PlatformApplication,
}

impl EventLoop {
    /// Run the application forever. When a new event occurs the callback passed in will be called.
    /// On MacOS the callback does not run on the main thread.
    #[cfg(not(target_arch = "wasm32"))]
    pub fn run<T>(mut self, callback: T)
    where
        T: 'static + FnMut(Event) + Send,
    {
        self.platform_application.run(Box::new(callback));
    }

    // Same as above but does not require Send
    #[cfg(target_arch = "wasm32")]
    pub fn run<T>(mut self, callback: T)
    where
        T: 'static + FnMut(Event),
    {
        self.platform_application.run(Box::new(callback));
    }
}
