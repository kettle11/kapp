use crate::{
    ApplicationMessage, Event, PlatformApplicationTrait, PlatformChannelTrait, PlatformWakerTrait,
};
pub struct PlatformApplication {}

impl PlatformApplicationTrait for PlatformApplication {
    type Waker = PlatformWaker;
    type Channel = PlatformChannel;

    fn new() -> (Self::Channel, Self) {
        (Self::Channel {}, Self {})
    }

    fn flush_events(&mut self) {}

    fn run(&mut self, callback: Box<dyn FnMut(Event)>) {
        super::event_loop_web::run(callback);
    }

    fn run_raw(&mut self, callback: Box<dyn FnMut(Event)>) {
        super::event_loop_web::run(callback);
    }

    fn get_waker(&self) -> Self::Waker {
        Self::Waker {}
    }
}

#[derive(Clone)]
pub struct PlatformWaker {}

impl PlatformWakerTrait for PlatformWaker {
    /// Call from any thread
    fn wake(&self) {}
    fn flush(&self) {}
}

#[derive(Clone)]
pub struct PlatformChannel {}

impl PlatformChannelTrait for PlatformChannel {
    fn send(&mut self, message: ApplicationMessage) {
        match message {
            ApplicationMessage::RequestFrame => super::event_loop_web::request_frame(),
            ApplicationMessage::SetWindowPosition { .. } => {}
            ApplicationMessage::SetWindowSize { .. } => {}
            ApplicationMessage::SetWindowTitle { .. } => {}
            ApplicationMessage::MinimizeWindow { .. } => {}
            ApplicationMessage::MaximizeWindow { .. } => {}
            ApplicationMessage::FullscreenWindow { .. } => {
                super::event_loop_web::request_fullscreen()
            }
            ApplicationMessage::RestoreWindow { .. } => unimplemented!(),
            ApplicationMessage::DropWindow { .. } => {}
            ApplicationMessage::SetMousePosition { .. } => unimplemented!(),
            ApplicationMessage::NewWindow { .. } => {}
            ApplicationMessage::Quit => {}
        }
    }
}
