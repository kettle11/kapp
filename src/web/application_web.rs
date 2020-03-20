use crate::application_message::{ApplicationMessage, ApplicationMessage::*};
use crate::platform_traits::*;
use crate::{Application, Event};
use std::sync::mpsc::*;

pub fn process_events(event_receiver: Receiver<ApplicationMessage>) {
    while let Ok(event) = event_receiver.try_recv() {
        match event {
            MinimizeWindow { window } => {}
            SetWindowPosition { window, x, y } => {}
            SetWindowSize {
                window,
                width,
                height,
            } => {}
            MaximizeWindow { .. } => {}
            FullscreenWindow { window } => {}
            RestoreWindow { .. } => unimplemented!(),
            DropWindow { .. } => unimplemented!(),
            RequestFrame { .. } => {}
            SetMousePosition { x, y } => {}
            Quit => {}
            NewWindow {
                window_parameters,
                response_channel,
            } => {}
        }
    }
}

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
    fn start_receiver<T>(&mut self, mut application: crate::Application, mut callback: T)
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
            ApplicationMessage::RequestFrame => super::event_loop_web::request_frame(),
            _ => {}
        }
    }
}
