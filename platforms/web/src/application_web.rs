use crate::application_message::ApplicationMessage::*;
use crate::platform_traits::*;
use crate::Application;

pub struct PlatformApplication {}

impl PlatformApplicationTrait for PlatformApplication {
    type PlatformWaker = PlatformWaker;
    type PlatformChannel = PlatformChannel;

    /// Only call from the main thread.
    fn new() -> (Self::PlatformChannel, Self) {
        (Self::PlatformChannel {}, Self {})
    }

    /// Only call from the main thread.
    fn flush_events(&mut self) {}

    /// Only call from the main thread.
    fn start_receiver<T>(&mut self, application: crate::Application, callback: T)
    where
        T: 'static + FnMut(&mut Application, crate::Event),
    {
        super::event_loop_web::run(application, callback);
    }

    /// Only call from the main thread.
    fn start_application(self) {}

    fn get_waker(&self) -> PlatformWaker {
        PlatformWaker {}
    }
}

#[derive(Clone)]
pub struct PlatformWaker {}

impl PlatformWakerTrait for PlatformWaker {
    /// Call from any thread
    fn wake(&self) {}
}

#[derive(Clone)]
pub struct PlatformChannel {}

impl PlatformChannelTrait for PlatformChannel {
    fn send(&mut self, message: crate::application_message::ApplicationMessage) {
        match message {
            RequestFrame => super::event_loop_web::request_frame(),
            SetWindowPosition { .. } => {}
            SetWindowSize { .. } => {}
            MinimizeWindow { .. } => {}
            MaximizeWindow { .. } => {}
            FullscreenWindow { .. } => super::event_loop_web::request_fullscreen(),
            RestoreWindow { .. } => unimplemented!(),
            DropWindow { .. } => {}
            SetMousePosition { .. } => unimplemented!(),
            NewWindow { .. } => {}
            Quit => {}
        }
    }
}
