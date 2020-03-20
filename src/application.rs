use crate::application_message::ApplicationMessage::*;
use crate::platform_traits::*;
use crate::{PlatformApplication, PlatformChannel, PlatformWaker};

#[derive(Clone)]
pub struct Application {
    pub platform_channel: PlatformChannel,
    pub application_waker: PlatformWaker,
}

impl Application {
    pub fn new() -> MainApplication {
        let (platform_channel, platform_application) = PlatformApplication::new();
        MainApplication {
            inner_application: Application {
                platform_channel,
                application_waker: platform_application.get_waker(),
            },
            platform_application,
        }
    }

    pub fn request_frame(&mut self) {
        self.platform_channel.send(RequestFrame);
        self.application_waker.wake();
    }

    pub fn quit(&mut self) {
        self.platform_channel.send(Quit);
    }

    pub fn set_mouse_position(&mut self, x: u32, y: u32) {
        self.platform_channel.send(SetMousePosition { x, y });
    }

    pub fn new_window(&mut self) -> crate::window_builder::WindowBuilder {
        crate::window_builder::WindowBuilder::new(self, None)
    }
}

/// An application handle only accessible from the main thread.
pub struct MainApplication {
    pub inner_application: Application, // Shouldn't be public
    pub platform_application: PlatformApplication, // Shouldn't be public
}

impl MainApplication {
    #[cfg(not(target_arch = "wasm32"))]
    pub fn run<T>(mut self, callback: T) -> !
    where
        T: 'static + FnMut(&mut Application, crate::Event) + Send,
    {
        // Async and other custom scenarios may take control of the event queue.
        // In that case don't start a receiver.
        // Platform specific code to run.
        self.platform_application
            .start_receiver(self.inner_application, callback);

        self.platform_application.start_application();
        unreachable!()
    }

    // Same as above but does not require Send
    #[cfg(target_arch = "wasm32")]
    pub fn run<T>(mut self, callback: T) -> !
    where
        T: 'static + FnMut(&mut Application, crate::Event),
    {
        // Async and other custom scenarios may take control of the event queue.
        // In that case don't start a receiver.
        // Platform specific code to run.
        self.platform_application
            .start_receiver(self.inner_application, callback);

        self.platform_application.start_application();
        unreachable!()
    }

    pub fn quit(&mut self) {
        self.inner_application.quit()
    }

    pub fn set_mouse_position(&mut self, x: u32, y: u32) {
        self.inner_application.set_mouse_position(x, y)
    }

    pub fn new_window(&mut self) -> crate::window_builder::WindowBuilder {
        crate::window_builder::WindowBuilder::new(
            &mut self.inner_application,
            Some(&mut self.platform_application),
        )
    }

    // Should this not be part of the public interface?
    /// If this instance of Application is on the main thread, process all application events.
    pub fn flush_events(&mut self) {
        self.platform_application.flush_events();
    }
}

// When the application is dropped, quit the program.
impl Drop for Application {
    fn drop(&mut self) {
        self.quit();
    }
}
