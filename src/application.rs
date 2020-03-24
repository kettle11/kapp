use crate::application_message::ApplicationMessage::*;
use crate::platform_traits::*;
use crate::{PlatformApplication, PlatformChannel, PlatformWaker};

#[derive(Clone)]
pub struct Application {
    pub platform_channel: PlatformChannel,
    pub application_waker: PlatformWaker,
}

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

    /// Blocks until the application has processed all events sent to it.
    pub fn flush_application_events(&mut self) {
        self.application_waker.flush();
    }

    pub fn new_window(&mut self) -> crate::window_builder::WindowBuilder {
        crate::window_builder::WindowBuilder::new(self)
    }
}

// When the application is dropped, quit the program.
impl Drop for Application {
    fn drop(&mut self) {
        self.quit();
    }
}

/// An application handle only accessible from the main thread.
pub struct EventLoop {
    pub platform_application: PlatformApplication, // Shouldn't be public
}

impl EventLoop {
    #[cfg(not(target_arch = "wasm32"))]
    pub fn run<T>(mut self, mut callback: T) -> !
    where
        T: 'static + FnMut(crate::Event) + Send,
    {
        let (send, receive) = std::sync::mpsc::channel();

        // When events are produced by the application send them to a channel
        let callback_wrapper = move |event| {
            send.send(event).unwrap();
        };

        // Receive the events from the channel and send them to the user code callback.
        std::thread::spawn(move || {
            while let Ok(event) = receive.recv() {
                callback(event);
            }
        });

        println!("Running");
        self.platform_application.run(callback_wrapper);

        unreachable!()
    }

    // Same as above but does not require Send
    #[cfg(target_arch = "wasm32")]
    pub fn run<T>(mut self, callback: T)
    where
        T: 'static + FnMut(crate::Event),
    {
        unimplemented!();
        self.platform_application.start_receiver(callback);

        self.platform_application.start_application();
    }
}
