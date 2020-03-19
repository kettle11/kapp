use crate::application_message::ApplicationMessage;
use crate::application_message::ApplicationMessage::*;
use crate::{PlatformApplication, PlatformApplicationWaker};
use std::sync::mpsc::*;

pub struct Application {
    pub program_to_application_send: Sender<ApplicationMessage>,
    pub application_waker: PlatformApplicationWaker,
}

impl Application {
    pub fn new() -> MainApplication {
        let (program_to_application_send, program_to_application_receive) =
            channel::<ApplicationMessage>();

        let platform_application = PlatformApplication::new(program_to_application_receive);
        MainApplication {
            inner_application: Application {
                program_to_application_send,
                application_waker: platform_application.get_waker(),
            },
            platform_application,
        }
    }

    pub fn request_frame(&mut self) {
        self.program_to_application_send.send(RequestFrame).unwrap();
        self.application_waker.wake();
    }

    pub fn quit(&self) {
        self.program_to_application_send.send(Quit).unwrap();
    }

    pub fn set_mouse_position(&self, x: u32, y: u32) {
        self.program_to_application_send
            .send(SetMousePosition { x, y })
            .unwrap();
    }

    pub fn new_window(&mut self) -> crate::window_builder::WindowBuilder {
        crate::window_builder::WindowBuilder::new(self, None)
    }
}

/// An application handle only accessible from the main thread.
pub struct MainApplication {
    inner_application: Application,
    platform_application: PlatformApplication,
}

impl MainApplication {
    /// This will never return.
    pub fn run<T>(self, callback: T) -> !
    where
        T: 'static + FnMut(&mut Application, crate::Event) + Send,
    {
        // Platform specific code to run.
        self.platform_application
            .run(self.inner_application, callback);
        unreachable!()
    }

    pub fn quit(&self) {
        self.inner_application.quit()
    }

    pub fn set_mouse_position(&self, x: u32, y: u32) {
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
